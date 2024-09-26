use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, msg::ExecuteMsg};

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMetadata { identity_data } => {
            exec::upsert_identity(deps, env, info, identity_data)
        }
        ExecuteMsg::StoreLoan {
            user_id,
            loan_id,
            loan,
        } => exec::store_loan(deps, env, info, user_id, loan_id, loan),
        ExecuteMsg::AssignLoansToReviewer { reviewer, loans } => {
            exec::assign_loans_to_reviewer(deps, info, reviewer, loans)
        }
        ExecuteMsg::UpdateLoanReviewStatus {
            user_id,
            loan_id,
            new_status,
        } => exec::update_loan_review_status(deps, env, user_id, loan_id, new_status),
    }
}

mod exec {
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};

    use crate::{
        error::ContractError,
        models::{IdentityMetadata, LoanData, ReviewStatus},
        states::{IDENTITIES, LOAN_STORAGE, REVIEWER_ASSIGNMENTS},
    };

    pub fn upsert_identity(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        identity_data: IdentityMetadata,
    ) -> Result<Response, ContractError> {
        let address = info.sender;

        let existing_identity = IDENTITIES.may_load(deps.storage, &address)?;

        if existing_identity.is_some() {
            // Identity exists, update it
            IDENTITIES.save(deps.storage, &address, &identity_data)?;
            Ok(Response::new().add_attribute("action", "update_metadata"))
        } else {
            // Identity does not exist, create new one
            IDENTITIES.save(deps.storage, &address, &identity_data)?;
            Ok(Response::new().add_attribute("action", "mint_identity"))
        }
    }

    // Function to store loan data
    pub fn store_loan(
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
        user_id: String,
        loan_id: String,
        mut loan: LoanData,
    ) -> Result<Response, ContractError> {
        // Set the creation date as the current block time
        loan.creation_date = env.block.time.seconds();

        LOAN_STORAGE.save(deps.storage, (&user_id.as_str(), &loan_id.as_str()), &loan)?;

        Ok(Response::new()
            .add_attribute("method", "store_loan")
            .add_attribute("user_id", user_id)
            .add_attribute("loan_id", loan_id))
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
