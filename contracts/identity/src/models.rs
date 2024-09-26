use core::fmt;
use std::collections::HashMap;

use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct IdentityMetadata {
    pub address: Addr,  // user wallet address
    pub name: String,   // user displayed nickname
    pub about: String,  // text about the user
    pub pic: String,    // ipfs image hash
    pub avatar: String, // 3d animated model ipfs hash
}

// Loan data structure with review status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoanData {
    pub amount: u128,
    pub borrower: String,
    pub interest_rate: String,
    pub duration: u64, // in months
    pub review_status: ReviewStatus, // Track the review status of each loan
    pub creation_date: u64,          // Unix timestamp when the loan was created
    pub approval_date: Option<u64>,  // Unix timestamp when the loan was approved
    pub rejection_date: Option<u64>, // Unix timestamp when the loan was rejected
}

// Define the possible review statuses for a loan
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, Hash)]
pub enum ReviewStatus {
    Pending,
    Approved,
    Reviewed,
    Rejected,
}


impl fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LoanStatistics {
    pub reviewer: Option<String>,
    pub pending_count: u64,
    pub rejected_this_month: u64,
    pub rejected_last_month: u64,
    pub approved_this_month: u64,
    pub approved_last_month: u64,
    pub average_time_to_process: Option<String>,  // in seconds
    pub month_wise_status_count: HashMap<String, HashMap<String, u64>>, // Month -> {Status -> Count}
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AllReviewerStatistics {
    pub total_pending: u64,
    pub total_approved: u64,
    pub total_rejected: u64,
    pub reviewers_stats: Vec<LoanStatistics>,
}