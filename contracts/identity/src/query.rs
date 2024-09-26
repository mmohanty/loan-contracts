use cosmwasm_std::{entry_point, to_json_binary, Binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserInfo { address } => to_json_binary(&query::query_identity(deps, address)?),
        QueryMsg::UserInfoAll {} => to_json_binary(&query::query_all_identities(deps)?),
        QueryMsg::GetLoansByStatus { status } => {
            to_json_binary(&query::query_loans_by_status(deps, status)?)
        }
        QueryMsg::GetLoansForUser { user_id } => {
            to_json_binary(&query::query_loans_for_user(deps, user_id)?)
        }
        QueryMsg::GetLoanDetails { user_id, loan_id } => {
            to_json_binary(&query::query_loan(deps, user_id, loan_id)?)
        }
        QueryMsg::GetLoansForReviewer { reviewer } => {
            to_json_binary(&query::query_loans_for_reviewer(deps, reviewer)?)
        }
        QueryMsg::GetLoansByDate {
            from_date,
            date_type,
        } => to_json_binary(&query::query_loans_by_date(deps, from_date, date_type)?),
        QueryMsg::GetLoanStatistics { reviewer } => {
            to_json_binary(&query::query_loan_statistics(deps, _env, reviewer)?)
        },
        QueryMsg::GetAllReviewerStatistics {} => {
            to_json_binary(&query::query_all_reviewer_statistics(deps, _env)?)
        }
    }
}

mod query {
    use std::collections::HashMap;

    use cosmwasm_std::{Addr, Deps, Env, StdResult};

    use crate::{
        models::{AllReviewerStatistics, IdentityMetadata, LoanData, LoanStatistics, ReviewStatus}, states::{IDENTITIES, LOAN_STORAGE, REVIEWER_ASSIGNMENTS}
    };

    pub fn query_loan(deps: Deps, user_id: String, loan_id: String) -> StdResult<LoanData> {
        let loan = LOAN_STORAGE
            .load(deps.storage, (&user_id, &loan_id));
        Ok(loan?)
    }
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

    // Function to query loan data by (user_id)
    pub fn query_loans_for_user(deps: Deps, user_id: String) -> StdResult<Vec<LoanData>> {
        // Use range to iterate over all the loans in storage
        let loans: Vec<LoanData> = LOAN_STORAGE
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending) // Iterate over the storage
            .filter_map(|item| {
                let ((stored_user_id, _loan_id), loan_data) = item.ok()?; // Extract the user_id and loan_id
                if stored_user_id == user_id {
                    Some(loan_data) // Keep only loans for the provided user_id
                } else {
                    None // Filter out loans that don't match the user_id
                }
            })
            .collect(); // Collect all the loans into a Vec<LoanData>

        Ok(loans)
    }

    // Function to query loans by their review status
    pub fn query_loans_by_status(
        deps: Deps,
        status: ReviewStatus,
    ) -> StdResult<Vec<(String, String)>> {
        let loans: Vec<(String, String)> = LOAN_STORAGE
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                let ((user_id, loan_id), loan) = item.ok()?;
                if loan.review_status == status {
                    Some((user_id.to_string(), loan_id.to_string()))
                } else {
                    None
                }
            })
            .collect();

        Ok(loans)
    }

    // Function to query loans assigned to a reviewer
    pub fn query_loans_for_reviewer(
        deps: Deps,
        reviewer: String,
    ) -> StdResult<Vec<(String, String)>> {
        let reviewer_addr = deps.api.addr_validate(&reviewer)?;
        let loans = REVIEWER_ASSIGNMENTS.load(deps.storage, &reviewer_addr)?;
        Ok(loans)
    }

    // Function to query loans by their creation, approval, or rejection date
    pub fn query_loans_by_date(
        deps: Deps,
        from_date: u64,
        date_type: String,
    ) -> StdResult<Vec<(String, String)>> {
        let loans: Vec<(String, String)> = LOAN_STORAGE
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                let ((user_id, loan_id), loan) = item.ok()?;
                match date_type.as_str() {
                    "created" if loan.creation_date >= from_date => {
                        Some((user_id.to_string(), loan_id.to_string()))
                    }
                    "approved" if loan.approval_date.unwrap_or(0) >= from_date => {
                        Some((user_id.to_string(), loan_id.to_string()))
                    }
                    "rejected" if loan.rejection_date.unwrap_or(0) >= from_date => {
                        Some((user_id.to_string(), loan_id.to_string()))
                    }
                    _ => None,
                }
            })
            .collect();

        Ok(loans)
    }

    pub fn query_loan_statistics(
        deps: Deps,
        env: Env,
        reviewer: Option<String>,
    ) -> StdResult<LoanStatistics> {
        let reviewer_addr = deps.api.addr_validate(&reviewer.unwrap_or_default())?;
    
        let loan_statistics = prepapre_loan_statistics(env.clone(), deps, reviewer_addr)?;
        Ok(loan_statistics)
    }

    fn prepapre_loan_statistics(env: Env, deps: Deps<'_>, reviewer_addr: Addr) -> Result<LoanStatistics, cosmwasm_std::StdError> {
        let current_timestamp = env.block.time.seconds();
        let month_seconds = 30 * 24 * 60 * 60;
        let current_month_start = current_timestamp - (current_timestamp % month_seconds);
        let last_month_start = current_month_start - month_seconds;
        let mut pending_count = 0;
        let mut rejected_this_month = 0;
        let mut rejected_last_month = 0;
        let mut approved_this_month = 0;
        let mut approved_last_month = 0;
        let mut total_processing_time: u64 = 0;
        let mut processed_loans_count = 0;
        let mut month_wise_status_count: HashMap<String, HashMap<String, u64>> = HashMap::new();
        if let Some(assigned_loans) = REVIEWER_ASSIGNMENTS.may_load(deps.storage, &reviewer_addr)? {
            for (user_id, loan_id) in assigned_loans {
                if let Some(loan) = LOAN_STORAGE.may_load(deps.storage, (&user_id, &loan_id))? {
                    // Count pending loans
                    if loan.review_status == ReviewStatus::Pending {
                        pending_count += 1;
                    }
    
                    // Process approval or rejection stats based on date
                    if let Some(approval_date) = loan.approval_date {
                        if approval_date >= current_month_start {
                            approved_this_month += 1;
                        } else if approval_date >= last_month_start {
                            approved_last_month += 1;
                        }
                        total_processing_time += approval_date - loan.creation_date;
                        processed_loans_count += 1;
                    }
    
                    if let Some(rejection_date) = loan.rejection_date {
                        if rejection_date >= current_month_start {
                            rejected_this_month += 1;
                        } else if rejection_date >= last_month_start {
                            rejected_last_month += 1;
                        }
                        total_processing_time += rejection_date - loan.creation_date;
                        processed_loans_count += 1;
                    }
    
                    // Month-wise count of loans by status
                    let loan_creation_month = format!("{}", loan.creation_date / month_seconds);
                    let status_count = month_wise_status_count
                        .entry(loan_creation_month)
                        .or_insert_with(HashMap::new);
                    *status_count.entry(loan.review_status.clone().to_string()).or_insert(0) += 1;
                }
            }
        }
        let average_time_to_process_float = if processed_loans_count > 0 {
            Some(total_processing_time as f64 / processed_loans_count as f64)
        } else {
            None
        };
        let average_time_to_process = average_time_to_process_float.map(|e: f64| e.to_string());
        let loan_statistics = LoanStatistics {
            reviewer: None,
            pending_count,
            rejected_this_month,
            rejected_last_month,
            approved_this_month,
            approved_last_month,
            average_time_to_process,
            month_wise_status_count,
        };
        Ok(loan_statistics)
    }
    
    pub fn query_all_reviewer_statistics(
        deps: Deps,
        env: Env,
    ) -> StdResult<AllReviewerStatistics> {
    
        let mut total_pending = 0;
        let mut total_approved = 0;
        let mut total_rejected = 0;
        let mut reviewers_stats: Vec<LoanStatistics> = vec![];
    
        // Iterate over all reviewers
        let all_reviewers: Vec<Addr> = REVIEWER_ASSIGNMENTS
            .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .map(|result| result.unwrap())
            .collect();
    
        // Process each reviewer
        for reviewer_addr in all_reviewers {

            let loan_statistics = prepapre_loan_statistics(env.clone(), deps, reviewer_addr)?;
    
            // Update totals
            total_pending += loan_statistics.pending_count;
            total_approved += loan_statistics.approved_this_month + loan_statistics.approved_last_month;
            total_rejected += loan_statistics.rejected_this_month + loan_statistics.rejected_last_month;
    
            // Append statistics for this reviewer
            reviewers_stats.push(loan_statistics);
        }
    
        Ok(AllReviewerStatistics {
            total_pending,
            total_approved,
            total_rejected,
            reviewers_stats,
        })
    }
}
