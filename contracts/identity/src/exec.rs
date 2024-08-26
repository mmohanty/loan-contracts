use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{error::ContractError, models::{IdentityMetadata, LoanData}, states::IDENTITIES, states::LOAN_DATA};

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

pub fn upsert_loan(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    loan_data: LoanData,
) -> Result<Response, ContractError> {

    let loan_number: String = loan_data.loan_number;

    let data = LoanData {
        loan_number: loan_number.to_string(),
        loan_amount: loan_data.loan_amount.to_string(),
        interest_rate: loan_data.interest_rate.to_string(),
        loan_duration: loan_data.loan_duration.to_string(), 
        loan_type: loan_data.loan_type.to_string(), 
        loan_status: loan_data.loan_status.to_string(),
        loan_owner: loan_data.loan_owner.to_string()
    };


    let existing_loan = LOAN_DATA.may_load(deps.storage, &loan_number)?;

    if existing_loan.is_some() {
        // Loan exists, update it
        LOAN_DATA.save(deps.storage, &loan_number, &data)?;
        Ok(Response::new().add_attribute("action", "update_loan_metadata"))
    } else {
        // Loan does not exist, create new one
        LOAN_DATA.save(deps.storage, &loan_number, &data)?;
        Ok(Response::new().add_attribute("action", "mint_loan"))
    }
}
