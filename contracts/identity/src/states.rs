use cosmwasm_std::Addr;
use cw_storage_plus::Map;

use crate::models::{IdentityMetadata, LoanData};

pub const IDENTITIES: Map<&Addr, IdentityMetadata> = Map::new("identities");


pub const LOAN_DATA:  Map<&str, LoanData> = Map::new("loan_data");
