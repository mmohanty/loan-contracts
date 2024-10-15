use cosmwasm_std::Timestamp;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MetaData {
    pub min: Option<i32>,
    pub max: Option<i32>,
    pub format: Option<String>,
    pub is_required: bool,
    pub is_unique: bool,
    pub is_editable: bool,
    pub field_type: FieldType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum FieldType {
    String,
    Integer,
    Decimal,
    Boolean,
    Date,
    URL,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Template {
    pub id: String,
    pub name: String,
    pub creator: String,
    pub loan_creators: Vec<String>,
    pub loan_reviewers: Vec<String>,
    pub fields: HashMap<String, MetaData>,
    pub loan_creator_approvals: Vec<String>,
    pub loan_reviewer_approvals: Vec<String>,
    pub status: SubmissionStatus,
    pub review_status: ReviewStatus,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum SubmissionStatus {
    Draft,
    Submitted,
    FullyApproved,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ReviewStatus {
    NotReviewed,
    PartialApproval,
    FullyApproved,
    Rejected { reviewer: String, comments: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoanAttribute {
    pub value: String,
    pub field_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Loan {
    pub id: String,
    pub template_id: String,
    pub creator: String,
    pub attributes: HashMap<String, LoanAttribute>,
    pub status: LoanStatus,
    pub created_at: Timestamp, // Timestamp when the loan was created
    pub updated_at: Option<Timestamp>, // Timestamp when the loan was last updated (optional)
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoanStatistics {
    pub total_loans: u64,
    pub pending_count: u64,
    pub returned_count: u64,
    pub approved_count: u64,
    pub rejected_count: u64,
    pub total_processing_time: Option<String>,
    pub avg_processing_time: Option<String>,  // Average time in seconds
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum LoanStatus {
    Draft,
    Submitted,
    Pending,    // Loan is pending review
    Returned { reviewer: String, comments: String  },   // Loan has been returned for changes
    Approved { reviewer: String, comments: Option<String> },
    Rejected { reviewer: String, comments: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoanDateQueryFilters {
    pub id: Option<String>,
    pub status: Option<LoanStatus>,
    pub creator: Option<String>,
    pub start_date: Option<Timestamp>,
    pub end_date: Option<Timestamp>,
    pub limit: Option<u32>,         // Pagination: max number of results
    pub offset: Option<u32>,        // Pagination: starting point for the results
}