use cosmwasm_std::{Addr, Deps, StdResult};

use crate::{models::{IdentityMetadata, LoanData}, states::{IDENTITIES, LOAN_DATA},};

pub fn query_identity(deps: Deps, address: Addr) -> StdResult<IdentityMetadata> {
    let identity = IDENTITIES.load(deps.storage, &address)?;
    Ok(identity)
}

pub fn query_all_identities(deps: Deps) -> StdResult<Vec<(Addr, IdentityMetadata)>> {
    let identities: StdResult<Vec<_>> = IDENTITIES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect();
    identities
}


pub fn query_all_loans(deps: Deps) -> StdResult<Vec<(String, LoanData)>> {
    let loans: StdResult<Vec<_>> = LOAN_DATA
    .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    .collect();
    loans
}
