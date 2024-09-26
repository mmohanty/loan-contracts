use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, identity, msg::ExecuteMsg};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMetadata { identity_data } => {
            identity::upsert_identity(deps, env, info, identity_data)
        }
        ExecuteMsg::CreateLoan {
            user_id,
            loan_requests,
        } => exec::create_loan(deps, env, info, user_id, loan_requests),
        ExecuteMsg::AssignLoansToReviewer { reviewer, loans } => {
            exec::assign_loans_to_reviewer(deps, info, reviewer, loans)
        }
        ExecuteMsg::UpdateLoanReviewStatus {
            user_id,
            loan_id,
            new_status,
        } => exec::update_loan_review_status(deps, env, user_id, loan_id, new_status),
        ExecuteMsg::CreateLoanTemplate {
            template_id,
            name,
            fields,
        } => exec::create_loan_template(deps, env, info, template_id, name, fields),
        ExecuteMsg::SubmitTemplateForReview {
            template_id,
            reviewer,
        } => exec::submit_template_for_review(deps, env, info, template_id, reviewer),
        ExecuteMsg::ReviewTemplate {
            template_id,
            approve,
        } => exec::review_template(deps, env, info, template_id, approve),
    }
}

mod exec {
    use std::collections::HashMap;

    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};
    use uuid::Uuid;

    use crate::{
        error::ContractError,
        models::{FieldType, LoanData, LoanRequest, LoanTemplate, ReviewStatus, ReviewTuple},
        states::{LOAN_STORAGE, REVIEWER_ASSIGNMENTS, TEMPLATE_REVIEWERS, USER_TEMPLATES},
    };

    pub fn create_loan_template(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        template_id: String,
        name: String,
        fields: HashMap<String, FieldType>, // Field definitions for the template
    ) -> Result<Response, ContractError> {
        let user_id = info.sender.to_string(); // Identify the user creating the template

        // Ensure that the template ID doesn't already exist for this user
        if USER_TEMPLATES.has(deps.storage, (&user_id, &template_id)) {
            return Err(ContractError::TemplateAlreadyExists {
                template_id: template_id.clone(),
            });
        }

        // Create the new loan template
        let template = LoanTemplate {
            id: template_id.clone(),
            name,
            fields,
            submitter: user_id.clone(),
            reviewer: None,                       // No reviewer assigned yet
            review_status: ReviewStatus::Pending, // Initially pending review
        };

        // Store the loan template in user-specific storage
        USER_TEMPLATES.save(deps.storage, (&user_id, &template_id), &template)?;

        // Return success response
        Ok(Response::new()
            .add_attribute("method", "create_loan_template")
            .add_attribute("template_id", template_id)
            .add_attribute("submitter", user_id)
            .add_attribute("status", "pending"))
    }

    pub fn submit_template_for_review(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        template_id: String, // The template ID to submit for review
        reviewer: String,    // The ID of the reviewer to assign
    ) -> Result<Response, ContractError> {
        let user_id = info.sender.to_string();

        // Load the template from the user's templates
        USER_TEMPLATES.update(deps.storage, (&user_id, &template_id), |maybe_template| {
            let mut template = maybe_template.ok_or_else(|| ContractError::TemplateNotFound {
                template_id: template_id.clone(),
            })?;

            // Ensure the template has not been reviewed already
            if template.review_status != ReviewStatus::Pending {
                return Err(ContractError::TemplateAlreadyReviewed {
                    template_id: template_id.clone(),
                });
            }

            // Assign the reviewer and update the template's status
            template.reviewer = Some(reviewer.clone());
            template.review_status = ReviewStatus::Pending;

            // Save the updated template
            Ok(template)
        })?;

        // Track the reviewer assignment
        let tuple = ReviewTuple{
            reviewer: reviewer.clone(),
            creater: user_id.clone(),
        };
        TEMPLATE_REVIEWERS.save(deps.storage, &template_id, &tuple)?;

        // Return success response
        Ok(Response::new()
            .add_attribute("method", "submit_template_for_review")
            .add_attribute("template_id", template_id)
            .add_attribute("reviewer", reviewer)
            .add_attribute("status", "pending"))
    }

    pub fn review_template(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        template_id: String, // The template ID to review
        approve: bool,       // True for approve, False for reject
    ) -> Result<Response, ContractError> {
        // Check that the template has been assigned to the reviewer
        let tuple = TEMPLATE_REVIEWERS.load(deps.storage, &template_id)?;
        if tuple.reviewer != info.sender.to_string() {
            return Err(ContractError::Unauthorized {});
        }

        // Load the template and update its review status
        USER_TEMPLATES.update(
            deps.storage,
            (&tuple.creater, &template_id),
            |maybe_template| -> StdResult<_> {
                let mut template =
                    maybe_template.ok_or_else(|| StdError::generic_err("Template not found"))?;

                // Approve or reject the template
                template.review_status = if approve {
                    ReviewStatus::Approved
                } else {
                    ReviewStatus::Rejected
                };

                // Save the updated template
                Ok(template)
            },
        )?;

        // Remove the reviewer assignment as the review is complete
        TEMPLATE_REVIEWERS.remove(deps.storage, &template_id);

        // Return a success response
        let status = if approve { "approved" } else { "rejected" };
        Ok(Response::new()
            .add_attribute("method", "review_template")
            .add_attribute("template_id", template_id)
            .add_attribute("status", status))
    }

    // Function to store loan data
    pub fn create_loan(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        user_id: String,
        loan_requests: Vec<LoanRequest>,
    ) -> Result<Response, ContractError> {
        let mut response = Response::new();

        for loan_request in loan_requests {
            let template_id = &loan_request.template_id;
            let values = loan_request.values;

            // Load the template
            let template =
                USER_TEMPLATES.load(deps.storage, (&user_id.as_str(), template_id.as_str()))?;

            // Ensure the template is approved before creating the loan
            if template.review_status != ReviewStatus::Approved {
                return Err(ContractError::TemplateNotApproved {
                    template_id: template_id.clone(),
                });
            }
            //Ensure that loan request contains fields that exist in the template
            if template.fields.len() != values.len() {
                return Err(ContractError::InvalidLoanRequest {});
            }

            // Validate the fields in the loan match the template
            for (field_name, field_type) in &template.fields {
                if let Some(value) = values.get(field_name) {
                    // Validate field types and constraints
                    match field_type {
                        FieldType::String { format, .. } => {
                            if let Some(f) = format {
                                let re = regex_lite::Regex::new(f).map_err(|_| {
                                    StdError::generic_err(format!(
                                        "Invalid format for field '{}'",
                                        field_name
                                    ))
                                })?;
                                if !re.is_match(value) {
                                    return Err(ContractError::InvalidFormat {
                                        field_name: field_name.clone(),
                                    });
                                }
                            }
                        }
                        FieldType::Number {
                            min_value,
                            max_value,
                            ..
                        } => {
                            let parsed_value = value.parse::<f64>().map_err(|_| {
                                StdError::generic_err(format!(
                                    "Field '{}' should be a number",
                                    field_name
                                ))
                            })?;
                            if let Some(min) = min_value {
                                if parsed_value < *&min.parse::<f64>().unwrap() {
                                    return Err(ContractError::InvalidNumberField {
                                        field_name: field_name.clone(),
                                    });
                                }
                            }
                            if let Some(max) = max_value {
                                if parsed_value > *&max.parse::<f64>().unwrap() {
                                    return Err(ContractError::InvalidNumberField {
                                        field_name: field_name.clone(),
                                    });
                                }
                            }
                        }
                        FieldType::Boolean { .. } => {
                            if value != "true" && value != "false" {
                                return Err(ContractError::InvalidFormat {
                                    field_name: field_name.clone(),
                                });
                            }
                        }
                        FieldType::Date { format, .. } => {
                            if let Some(f) = format {
                                let re = regex_lite::Regex::new(f).map_err(|_| {
                                    StdError::generic_err(format!(
                                        "Invalid format for field '{}'",
                                        field_name
                                    ))
                                })?;
                                if !re.is_match(value) {
                                    return Err(ContractError::InvalidFormat {
                                        field_name: field_name.clone(),
                                    });
                                }
                            } else {
                                if value.parse::<u64>().is_err() {
                                    return Err(ContractError::InvalidFormat {
                                        field_name: field_name.clone(),
                                    });
                                }
                            }
                        }
                    }
                } else {
                    return Err(ContractError::MissingField {
                        field_name: field_name.clone(),
                    });
                }
            }

            // Generate a unique loan ID using the UUID crate
            let loan_id = Uuid::new_v4().to_string();

            // Create the loan data
            let loan = LoanData {
                loan_id: loan_id.clone(),
                template_id: template_id.clone(),
                values,
                review_status: ReviewStatus::Pending,
                creation_date: env.block.time.seconds(),
                approval_date: None,
                rejection_date: None,
            };

            // Save the loan to storage (use a composite key of user ID and loan ID)
            LOAN_STORAGE
                .save(
                    deps.storage,
                    (&_info.sender.to_string(), &loan.loan_id),
                    &loan,
                )
                .map_err(ContractError::from)?;

            // Add success attribute for each loan
            response = response.add_attribute("created_loan_id", loan_id);
        }

        Ok(response.add_attribute("method", "create_loans"))
    }

    // Function to assign multiple loans to a reviewer
    pub fn assign_loans_to_reviewer(
        deps: DepsMut,
        _info: MessageInfo,
        reviewer: String,
        loans: Vec<(String, String)>,
    ) -> Result<Response, ContractError> {
        let reviewer_addr = deps.api.addr_validate(&reviewer.as_str())?;

        // Save the list of loans assigned to this reviewer
        REVIEWER_ASSIGNMENTS.save(deps.storage, &reviewer_addr, &loans)?;

        Ok(Response::new()
            .add_attribute("method", "assign_loans_to_reviewer")
            .add_attribute("reviewer", reviewer.as_str()))
    }

    // Function to update the review status of a loan and set appropriate dates
    pub fn update_loan_review_status(
        deps: DepsMut,
        env: Env,
        user_id: String,
        loan_id: String,
        new_status: ReviewStatus,
    ) -> Result<Response, ContractError> {
        LOAN_STORAGE.update(
            deps.storage,
            (&user_id, &loan_id),
            |maybe_loan: Option<LoanData>| -> StdResult<_> {
                // If loan not found, return a StdError instead of ContractError
                let mut loan = maybe_loan.ok_or_else(|| StdError::generic_err("Loan not found"))?;
                loan.review_status = new_status.clone();

                // Set the approval or rejection date based on the new status
                match new_status {
                    ReviewStatus::Approved => loan.approval_date = Some(env.block.time.seconds()),
                    ReviewStatus::Rejected => loan.rejection_date = Some(env.block.time.seconds()),
                    _ => {}
                }

                Ok(loan)
            },
        )?;

        Ok(Response::new()
            .add_attribute("method", "update_loan_review_status")
            .add_attribute("user_id", user_id)
            .add_attribute("loan_id", loan_id))
    }
}
