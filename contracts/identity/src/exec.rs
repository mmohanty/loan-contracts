use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdResult};
use exec::{approve_template, create_loan, create_template, review_loan, submit_loan, submit_template};

use crate::msg::ExecuteMsg;


#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::CreateTemplate {
            id,
            name,
            loan_creators,
            loan_reviewers,
            fields,
        } => create_template(deps, info, id, name, loan_creators, loan_reviewers, fields),
        ExecuteMsg::SubmitTemplate { id } => submit_template(deps, info, id),
        ExecuteMsg::ApproveTemplate { id, as_creator } => approve_template(deps, info, id, as_creator),
        ExecuteMsg::CreateLoan { id, template_id, attributes } => create_loan(deps, env, info, id, template_id, attributes),
        ExecuteMsg::SubmitLoan { loan_id } => submit_loan(deps, info, loan_id),
        ExecuteMsg::ReviewLoan { loan_id, approve, return_loan,comments } => review_loan(deps, env, info, loan_id, approve, return_loan, comments),
    }
}

mod exec {
    use std::collections::HashMap;

    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};

    use crate::{models::{Loan, LoanAttribute, LoanStatus, MetaData, ReviewStatus, SubmissionStatus, Template}, states::{LOAN_STORE, TEMPLATE_STORE}};

    // Create a new template
pub fn create_template(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
    name: String,
    loan_creators: Vec<String>,
    loan_reviewers: Vec<String>,
    fields: HashMap<String, MetaData>,
) -> StdResult<Response> {

    let template = Template {
        id: id.clone(),
        name,
        creator: info.sender.to_string(),
        loan_creators,
        loan_reviewers,
        fields,
        loan_creator_approvals: vec![],
        loan_reviewer_approvals: vec![],
        status: SubmissionStatus::Draft,
        review_status: ReviewStatus::NotReviewed,
    };

    TEMPLATE_STORE.save(deps.storage, id.clone(), &template)?;

    Ok(Response::new()
        .add_attribute("method", "create_template")
        .add_attribute("template_id", id))
}

// Submit a template for approval
pub fn submit_template(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
) -> StdResult<Response> {
    let mut template = TEMPLATE_STORE.load(deps.storage, id.clone())?;

    if !template.creator.eq(&info.sender.to_string()){
        return Err(StdError::generic_err("Unauthorized to submit template"));
    }

    template.status = SubmissionStatus::Submitted;
    TEMPLATE_STORE.save(deps.storage, id.clone(), &template)?;

    Ok(Response::new()
        .add_attribute("method", "submit_template")
        .add_attribute("template_id", id))
}

// Approve a template by a creator or a reviewer
pub fn approve_template(
    deps: DepsMut,
    info: MessageInfo,
    id: String,
    as_creator: bool,
) -> StdResult<Response> {
    let mut template = TEMPLATE_STORE.load(deps.storage, id.clone())?;

    if as_creator && !template.loan_creators.contains(&info.sender.to_string()) {
        return Err(StdError::generic_err("Unauthorized: Not a creator"));
    }

    if !as_creator && !template.loan_reviewers.contains(&info.sender.to_string()) {
        return Err(StdError::generic_err("Unauthorized: Not a reviewer"));
    }

    if as_creator {
        if template.loan_creator_approvals.contains(&info.sender.to_string()) {
            return Err(StdError::generic_err("Already approved as a creator"));
        }
        template.loan_creator_approvals.push(info.sender.to_string());
    } else {
        if template.loan_reviewer_approvals.contains(&info.sender.to_string()) {
            return Err(StdError::generic_err("Already approved as a reviewer"));
        }
        template.loan_reviewer_approvals.push(info.sender.to_string());
    }

    let all_creators_approved = template.loan_creator_approvals.len() == template.loan_creators.len();
    let all_reviewers_approved = template.loan_reviewer_approvals.len() == template.loan_reviewers.len();

    if all_creators_approved && all_reviewers_approved {
        template.status = SubmissionStatus::FullyApproved;
        template.review_status = ReviewStatus::FullyApproved;
    } else {
        template.review_status = ReviewStatus::PartialApproval;
    }

    TEMPLATE_STORE.save(deps.storage, id.clone(), &template)?;

    Ok(Response::new()
        .add_attribute("method", "approve_template")
        .add_attribute("template_id", id)
        .add_attribute("approved_as", if as_creator { "creator" } else { "reviewer" }))
}

// Create a loan based on an approved template
pub fn create_loan(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    loan_id: String,
    template_id: String,
    attributes: HashMap<String, LoanAttribute>,
) -> StdResult<Response> {
    let template = TEMPLATE_STORE.load(deps.storage, template_id.clone())?;

    if template.status != SubmissionStatus::FullyApproved {
        return Err(StdError::generic_err("Template not fully approved"));
    }

    for (field_name, _) in &template.fields {
        if !attributes.contains_key(field_name) {
            return Err(StdError::generic_err(format!("Missing field: {}", field_name)));
        }
    }

    let loan = Loan {
        id: loan_id.clone(),
        template_id,
        creator: info.sender.to_string(),
        attributes,
        status: LoanStatus::Draft,
        created_at: env.block.time, // Set the creation time using env.block.time
        updated_at: None, // No update yet
    };

    LOAN_STORE.save(deps.storage, loan_id.clone(), &loan)?;

    Ok(Response::new()
        .add_attribute("method", "create_loan")
        .add_attribute("loan_id", loan_id))
}

// Submit a loan for review
pub fn submit_loan(
    deps: DepsMut,
    info: MessageInfo,
    loan_id: String,
) -> StdResult<Response> {
    let mut loan = LOAN_STORE.load(deps.storage, loan_id.clone())?;

    if loan.creator != info.sender.to_string() {
        return Err(StdError::generic_err("Unauthorized: Not the loan creator"));
    }

    loan.status = LoanStatus::Submitted;
    LOAN_STORE.save(deps.storage, loan_id.clone(), &loan)?;

    Ok(Response::new()
        .add_attribute("method", "submit_loan")
        .add_attribute("loan_id", loan_id))
}

// Approve or reject a loan by a reviewer
pub fn review_loan(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    loan_id: String,
    approve: bool,
    return_loan: bool,
    comments: Option<String>,
) -> StdResult<Response> {
    let mut loan = LOAN_STORE.load(deps.storage, loan_id.clone())?;

    if loan.status != LoanStatus::Submitted {
        return Err(StdError::generic_err("Loan is not submitted"));
    }

    if approve {
        loan.status = LoanStatus::Approved {
            reviewer: info.sender.to_string(),
            comments,
        };
        loan.updated_at = Some(env.block.time); // Set the updated time when approved
    }
    // Handle loan returning
    else if return_loan {
        loan.status = LoanStatus::Returned {
            reviewer: info.sender.to_string(),
            comments: comments.unwrap_or_else(|| "No comments provided".to_string()),
        };
        loan.updated_at = Some(env.block.time);  // Set the updated time when returned
    }  
    else {
        loan.status = LoanStatus::Rejected {
            reviewer: info.sender.to_string(),
            comments: comments.unwrap_or_else(|| "No comments provided".to_string()),
        };
        loan.updated_at = Some(env.block.time); // Set the updated time when rejected
    }

    LOAN_STORE.save(deps.storage, loan_id.clone(), &loan)?;

    Ok(Response::new()
        .add_attribute("method", "review_loan")
        .add_attribute("loan_id", loan_id)
        .add_attribute("status", if approve { "approved" } else { "rejected" }))
}
}
