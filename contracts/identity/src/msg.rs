use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::{IdentityMetadata, LoanData};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateMetadata { identity_data: IdentityMetadata },
    UpdateLoandata{loan_data: LoanData}
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IdentityMetadata)]
    UserInfo { address: Addr },
    #[returns(Vec<(Addr, IdentityMetadata)>)]
    UserInfoAll {},
    #[returns(Vec<(String, LoanData)>)]
    LoanDataAll{},
}
