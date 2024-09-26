use std::collections::HashMap;

use crate::models::{
    AllReviewerStatistics, FieldType, IdentityMetadata, LoanData, LoanRequest, LoanStatistics, LoanTemplate, ReviewStatus
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateMetadata {
        identity_data: IdentityMetadata,
    },
    CreateLoan {
        user_id: String,
        loan_requests: Vec<LoanRequest>,
    },
    AssignLoansToReviewer {
        reviewer: String,
        loans: Vec<(String, String)>,
    },
    UpdateLoanReviewStatus {
        user_id: String,
        loan_id: String,
        new_status: ReviewStatus,
    },
    CreateLoanTemplate {
        template_id: String,
        name: String,
        fields: HashMap<String, FieldType>,
    },
    SubmitTemplateForReview {
        template_id: String,
        reviewer: String,
    },
    ReviewTemplate {
        template_id: String,
        approve: bool,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(IdentityMetadata)]
    UserInfo { address: Addr },
    #[returns(Vec<(Addr, IdentityMetadata)>)]
    UserInfoAll {},
    #[returns(Vec<LoanData>)]
    GetLoansForUser { user_id: String },
    #[returns(LoanData)]
    GetLoanDetails { user_id: String, loan_id: String },
    #[returns(Vec<LoanData>)]
    GetLoansForReviewer { reviewer: String },
    #[returns(Vec<LoanData>)]
    GetLoansByStatus { status: ReviewStatus },
    #[returns(Vec<LoanData>)]
    GetLoansByDate {
        from_date: u64,    // Unix timestamp for filtering loans
        date_type: String, // "created", "approved", or "rejected"
    },
    #[returns(LoanStatistics)]
    GetLoanStatistics { reviewer: Option<String> },
    #[returns(AllReviewerStatistics)]
    GetAllReviewerStatistics {},

    #[returns(Vec<LoanTemplate>)]
    GetUserTemplates { user_id: String },

    #[returns(Vec<LoanTemplate>)]
    GetReviewerTemplates { reviewer: String },
}
