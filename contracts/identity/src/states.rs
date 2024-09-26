use cosmwasm_std::Addr;
use cw_storage_plus::Map;

use crate::models::{IdentityMetadata, LoanData};

pub const IDENTITIES: Map<&Addr, IdentityMetadata> = Map::new("identities");


// Map to store loan data with (user_id, loan_id) as composite key
pub const LOAN_STORAGE: Map<(&str, &str), LoanData> = Map::new("loans");


// Map to store assignments of loans to reviewers
pub const REVIEWER_ASSIGNMENTS: Map<&Addr, Vec<(String, String)>> = Map::new("assignments");

