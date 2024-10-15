use std::collections::HashMap;

use cosmwasm_schema::QueryResponses;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::{ Loan, LoanAttribute, LoanDateQueryFilters, LoanStatistics, MetaData, Template};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    CreateTemplate {
        id: String,
        name: String,
        loan_creators: Vec<String>,
        loan_reviewers: Vec<String>,
        fields: HashMap<String, MetaData>,
    },
    SubmitTemplate { id: String, },
    ApproveTemplate {
        id: String,
        as_creator: bool,

    },
    CreateLoan {
        id: String,
        template_id: String,
        attributes: HashMap<String, LoanAttribute>,
    },
    SubmitLoan { loan_id: String },
    ReviewLoan { loan_id: String, approve: bool, return_loan: bool, comments: Option<String> },

    
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
pub enum QueryMsg {

    // #[returns(Loan)]
    // QueryLoan { id: String },
    // #[returns(Vec<Loan>)]
    // QueryLoansForUser { user: String },
    // #[returns(Vec<Loan>)]
    // QueryLoansByStatus { status: LoanStatus },
    #[returns(Vec<Loan>)]
    QueryLoans { filters: LoanDateQueryFilters },
    #[returns(LoanStatistics)]
    QueryLoanStatistics {},
    #[returns(Vec<Template>)]
    QueryTemplatesForReviewer { reviewer: String },
    #[returns(Vec<Template>)]
    QueryUserTemplates { creator: String },   
}
