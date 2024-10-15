use cosmwasm_std::{entry_point, Binary, Deps, Env, StdResult};
use query::{query_loan_statistics, query_loans, query_templates_for_reviewer, query_user_templates};

use crate::msg::QueryMsg;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        //QueryMsg::QueryLoan { id } => query_loan(deps, id),
        //QueryMsg::QueryLoansForUser { user } => query_loans_for_user(deps, user),
        //QueryMsg::QueryLoansByStatus { status } => query_loans_by_status(deps, status),
        QueryMsg::QueryLoans {filters } => query_loans(deps, filters),
        QueryMsg::QueryLoanStatistics {} => query_loan_statistics(deps),
        QueryMsg::QueryTemplatesForReviewer { reviewer } => query_templates_for_reviewer(deps, reviewer),
        QueryMsg::QueryUserTemplates { creator } => query_user_templates(deps, creator),
    }
}

mod query {

    use cosmwasm_std::{to_json_binary, Binary, Deps,  Order, StdResult};

    use crate::{models::{Loan, LoanDateQueryFilters, LoanStatistics, LoanStatus, Template}, states::{LOAN_STORE, TEMPLATE_STORE}};

    // pub fn query_loan(deps: Deps, id: String) -> StdResult<Binary> {
    //     let loan = LOAN_STORE.load(deps.storage, id.clone())?;
    //     to_json_binary(&loan)
    // }
    
    // pub fn query_loans_for_user(deps: Deps, user: String) -> StdResult<Binary> {
    //     let loans: Vec<Loan> = LOAN_STORE
    //         .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    //         .filter_map(|item| item.ok()) // Extract only valid loans
    //         .filter(|(_, loan)| loan.creator == user) // Filter by creator
    //         .map(|(_, loan)| loan) // Return only the loan data
    //         .collect();
    
    //         to_json_binary(&loans)
    // }

    // pub fn query_loans_by_status(deps: Deps, status: LoanStatus) -> StdResult<Binary> {
    //     let loans: Vec<Loan> = LOAN_STORE
    //         .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
    //         .filter_map(|item| item.ok())
    //         .filter(|(_, loan)| loan.status == status)
    //         .map(|(_, loan)| loan)
    //         .collect();
    
    //     to_json_binary(&loans)
    // }
    

    pub fn query_loans(deps: Deps, 
            filters: LoanDateQueryFilters, // Additional filters for status, creator, etc.
        ) -> StdResult<Binary> {
        let mut loans: Vec<Loan> = LOAN_STORE
        .range(deps.storage, None, None, Order::Ascending) // Fetch loans in ascending order
        .filter_map(|item| item.ok())                      // Filter out any errors
        .filter(|(_, loan)| {
            // Apply date filter if provided
            let start_date_match = filters.start_date.map_or(true, |start| loan.created_at >= start);
            let end_date_match = filters.end_date.map_or(true, |end| loan.created_at <= end);

            // Apply id filter if provided
            let id_match = filters.id.as_ref().map_or(true, |id| loan.id == *id);

            // Apply status filter if provided
            let status_match = filters.status.as_ref().map_or(true, |status| loan.status == *status);

            // Apply creator filter if provided
            let creator_match = filters.creator.as_ref().map_or(true, |creator| loan.creator == *creator);

            // Combine all filters
            start_date_match && end_date_match && status_match && creator_match && id_match
        })
        .map(|(_, loan)| loan)
        .collect();

    // Apply pagination: limit and offset
    let offset = filters.offset.unwrap_or(0) as usize;
    let limit = filters.limit.unwrap_or(100) as usize;
    loans = loans.into_iter().skip(offset).take(limit).collect();

    // Serialize to binary
    to_json_binary(&loans)
    }
    
    pub fn query_loan_statistics(deps: Deps) -> StdResult<Binary> {
        let mut statistics = LoanStatistics {
            total_loans: 0,
            pending_count: 0,
            returned_count: 0,
            approved_count: 0,
            rejected_count: 0,
            avg_processing_time: None,
            total_processing_time: None,
        };

        let mut total_processing_time: u64 = 0;
        let mut processed_loan_count: u64 = 0;
    
        LOAN_STORE
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| item.ok())
            .for_each(|(_, loan)| {
                statistics.total_loans += 1;
    
                match loan.status {
                    LoanStatus::Pending => {
                        statistics.pending_count += 1;
                    }
                    LoanStatus::Returned{ .. } => {
                        statistics.returned_count += 1;
                    }
                    LoanStatus::Approved { .. } => {
                        statistics.approved_count += 1;
                    }
                    LoanStatus::Rejected { .. } => {
                        statistics.rejected_count += 1;
                    }
                    _ => {}
                }

                // Calculate processing time for loans that have been approved, returned, or rejected
                if let Some(updated_at) = loan.updated_at {
                    let processing_time = updated_at.seconds() - loan.created_at.seconds();
                    total_processing_time += processing_time;
                    processed_loan_count += 1;
                }
            });
            statistics.avg_processing_time = Some((total_processing_time/processed_loan_count).to_string());
            statistics.total_processing_time = Some(total_processing_time.to_string());
        to_json_binary(&statistics)
    }
    

    pub fn query_templates_for_reviewer(deps: Deps, reviewer: String) -> StdResult<Binary> {
        let templates: Vec<Template> = TEMPLATE_STORE
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| item.ok())
            .filter(|(_, template)| template.loan_reviewers.contains(&reviewer))
            .map(|(_, template)| template)
            .collect();
    
        to_json_binary(&templates)
    }
    

    pub fn query_user_templates(deps: Deps, creator: String) -> StdResult<Binary> {
        let templates: Vec<Template> = TEMPLATE_STORE
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| item.ok())
            .filter(|(_, template)| template.loan_creators.contains(&creator))
            .map(|(_, template)| template)
            .collect();
    
        to_json_binary(&templates)
    }
    
    
}
