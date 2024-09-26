#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::{attr, from_json, Addr};

    use crate::exec::execute;
    use crate::models::{AllReviewerStatistics, IdentityMetadata, LoanData, LoanStatistics, ReviewStatus};
    use crate::msg::{ExecuteMsg, QueryMsg};
    use crate::query::query;
    use crate::states::{IDENTITIES, LOAN_STORAGE, REVIEWER_ASSIGNMENTS};

    #[test]
    fn test_mint_identity() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("creator"), &[]);

        let metadata = IdentityMetadata {
            name: "Alice".to_string(),
            pic: "ipfs://pic".to_string(),
            address: Addr::unchecked("cosmos1...".to_string()),
            about: "About Alice".to_string(),
            avatar: "ipfs://avatar".to_string(),
        };

        let msg = ExecuteMsg::UpdateMetadata { identity_data: metadata.clone() };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg)
            .unwrap();
        assert_eq!(res.attributes[0].value, "mint_identity");

        let stored_metadata = IDENTITIES.load(&deps.storage, &info.sender).unwrap();
        assert_eq!(stored_metadata, metadata);
    }

    #[test]
    fn test_update_metadata() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("creator"), &[]);

        let metadata = IdentityMetadata {
            name: "Alice".to_string(),
            pic: "ipfs://pic".to_string(),
            address: Addr::unchecked("cosmos1...".to_string()),
            about: "About Alice".to_string(),
            avatar: "ipfs://avatar".to_string(),
        };

        let msg = ExecuteMsg::UpdateMetadata { identity_data: metadata.clone() };
        let _ = execute(deps.as_mut(), env.clone(), info.clone(), msg)
            .unwrap();

        let updated_metadata = IdentityMetadata {
            name: "Alice Updated".to_string(),
            pic: "ipfs://newpic".to_string(),
            address: Addr::unchecked("cosmos1...".to_string()),
            about: "Updated About Alice".to_string(),
            avatar: "ipfs://newavatar".to_string(),
        };

        let msg = ExecuteMsg::UpdateMetadata { identity_data: metadata.clone() };
        let res = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            msg,
        )
        .unwrap();
        assert_eq!(res.attributes[0].value, "update_metadata");

        let stored_metadata = IDENTITIES.load(&deps.storage, &info.sender).unwrap();
        assert_ne!(stored_metadata, updated_metadata);
    }

    #[test]
    fn test_query_identity() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = message_info(&Addr::unchecked("creator"), &[]);

        let metadata = IdentityMetadata {
            name: "Alice".to_string(),
            pic: "ipfs://pic".to_string(),
            address: Addr::unchecked("cosmos1...".to_string()),
            about: "About Alice".to_string(),
            avatar: "ipfs://avatar".to_string(),
        };

        let msg = ExecuteMsg::UpdateMetadata { identity_data: metadata.clone() };
        let _ = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();//exec::upsert_identity(   deps.as_mut(), env.clone(), info.clone(), metadata.clone())

        let query_message = QueryMsg::UserInfo { address: info.sender.clone() };
        let query_response = query(deps.as_ref(), env.clone(), query_message).unwrap();
        let res: IdentityMetadata = from_json(&query_response).unwrap();

        assert_eq!(res, metadata);
    }

    #[test]
    fn test_query_all_identities() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info1 = message_info(&Addr::unchecked("creator1".to_string()), &[]);
        let info2 = message_info(&Addr::unchecked("creator2".to_string()), &[]);

        let metadata1 = IdentityMetadata {
            name: "Alice".to_string(),
            pic: "ipfs://pic1".to_string(),
            address: Addr::unchecked("cosmos1...".to_string()),
            about: "About Alice".to_string(),
            avatar: "ipfs://avatar1".to_string(),
        };

        let metadata2 = IdentityMetadata {
            name: "Bob".to_string(),
            pic: "ipfs://pic2".to_string(),
            address: Addr::unchecked("cosmos2...".to_string()),
            about: "About Bob".to_string(),
            avatar: "ipfs://avatar2".to_string(),
        };
        let msg1 = ExecuteMsg::UpdateMetadata { identity_data: metadata1.clone() };

        let _ = execute(deps.as_mut(), env.clone(), info1.clone(), msg1)
            .unwrap();

        let msg2 = ExecuteMsg::UpdateMetadata { identity_data: metadata2.clone() };
        let _ = execute(deps.as_mut(), env.clone(), info2.clone(), msg2)
            .unwrap();

        let query_message = QueryMsg::UserInfoAll { };
        let query_response = query(deps.as_ref(), env.clone(), query_message).unwrap();
        let res: Vec<(Addr, IdentityMetadata)> = from_json(&query_response).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], (info1.sender.clone(), metadata1));
        assert_eq!(res[1], (info2.sender.clone(), metadata2));
    }

    
    #[test]
fn test_store_loan() {
    let mut deps = mock_dependencies();

    let info = message_info(&Addr::unchecked("creator"), &[]);
    let env = mock_env();

    let loan = LoanData {
        amount: 100000,
        borrower: "John Doe".to_string(),
        interest_rate: "5.0".to_string(),
        duration: 12,
        review_status: ReviewStatus::Pending,
        creation_date: 0,
        approval_date: Some(10),
        rejection_date: Some(10),
    };

    let msg = ExecuteMsg::StoreLoan {
        user_id: "user1".to_string(),
        loan_id: "loan1".to_string(),
        loan: loan.clone(),
    };

    // Call execute_store_loan
    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Assert response attributes
    assert_eq!(res.attributes, vec![attr("method", "store_loan"), attr("user_id", "user1"), attr("loan_id", "loan1")]);

    // Load the loan from storage and verify it
    let stored_loan = LOAN_STORAGE.load(&deps.storage, ("user1", "loan1")).unwrap();
    assert_eq!(stored_loan.amount, loan.amount);
    assert_eq!(stored_loan.borrower, loan.borrower);
    assert_eq!(stored_loan.interest_rate, loan.interest_rate);
    assert_eq!(stored_loan.duration, loan.duration);
}


    
#[test]
fn test_assign_loans_to_reviewer() {

    let mut deps = mock_dependencies();


    // Reviewer address
    let verifier:Addr = deps.api.addr_make("reviewer1");


    let info = message_info(&Addr::unchecked("creator"), &[]);
    let msg = ExecuteMsg::AssignLoansToReviewer {
        reviewer: verifier.to_string(),
        loans: vec![("user1".to_string(), "loan1".to_string()), ("user2".to_string(), "loan2".to_string())],
    };

    // Execute assign loans to reviewer
    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Assert response attributes
    assert_eq!(res.attributes, vec![attr("method", "assign_loans_to_reviewer"), attr("reviewer", verifier.to_string())]);

    // Verify the loans have been assigned to the reviewer
    let assigned_loans = REVIEWER_ASSIGNMENTS.load(&deps.storage, &verifier).unwrap();
    assert_eq!(assigned_loans.len(), 2);
    assert_eq!(assigned_loans[0], ("user1".to_string(), "loan1".to_string()));
    assert_eq!(assigned_loans[1], ("user2".to_string(), "loan2".to_string()));
}

#[test]
fn test_update_loan_review_status() {
    let mut deps = mock_dependencies();

    // First store a loan
    let info = message_info(&Addr::unchecked("creator"), &[]);
    let env = mock_env();
    let loan = LoanData {
        amount: 100000,
        borrower: "John Doe".to_string(),
        interest_rate: "5.0".to_string(),
        duration: 12,
        review_status: ReviewStatus::Pending,
        creation_date: env.block.time.seconds(),
        approval_date: None,
        rejection_date: None,
    };

    let msg = ExecuteMsg::StoreLoan {
        user_id: "user1".to_string(),
        loan_id: "loan1".to_string(),
        loan: loan.clone(),
    };

    execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Now update the loan's status to Approved
    let msg = ExecuteMsg::UpdateLoanReviewStatus {
        user_id: "user1".to_string(),
        loan_id: "loan1".to_string(),
        new_status: ReviewStatus::Approved,
    };

    let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
    assert_eq!(res.attributes, vec![attr("method", "update_loan_review_status"), attr("user_id", "user1"), attr("loan_id", "loan1")]);

    // Verify the loan has been updated
    let updated_loan = LOAN_STORAGE.load(&deps.storage, ("user1", "loan1")).unwrap();
    assert_eq!(updated_loan.review_status, ReviewStatus::Approved);
    assert!(updated_loan.approval_date.is_some());
}

#[test]
fn test_query_loan() {
    let mut deps = mock_dependencies();

    // First store a loan
    let info = message_info(&Addr::unchecked("creator"), &[]);
    let env = mock_env();
    let loan = LoanData {
        amount: 100000,
        borrower: "John Doe".to_string(),
        interest_rate: "5.0".to_string(),
        duration: 12,
        review_status: ReviewStatus::Pending,
        creation_date: env.block.time.seconds(),
        approval_date: None,
        rejection_date: None,
    };

    let msg = ExecuteMsg::StoreLoan {
        user_id: "user1".to_string(),
        loan_id: "loan1".to_string(),
        loan: loan.clone(),
    };

    execute(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Now query the loan
    let res = query(deps.as_ref(), env, QueryMsg::GetLoanDetails {
        user_id: "user1".to_string(),
        loan_id: "loan1".to_string(),
    }).unwrap();

    let queried_loan: LoanData = from_json(&res).unwrap();
    assert_eq!(queried_loan.amount, loan.amount);
    assert_eq!(queried_loan.borrower, loan.borrower);
    assert_eq!(queried_loan.review_status, loan.review_status);
}

#[test]
fn test_query_loans_for_user() {
    let mut deps = mock_dependencies();

    let info = message_info(&Addr::unchecked("creator"), &[]);
    let env = mock_env();

    // Store multiple loans for the user
    let loan1 = LoanData {
        amount: 100000,
        borrower: "John Doe".to_string(),
        interest_rate: "5.0".to_string(),
        duration: 12,
        review_status: ReviewStatus::Pending,
        creation_date: env.block.time.seconds(),
        approval_date: None,
        rejection_date: None,
    };

    let loan2 = LoanData {
        amount: 200000,
        borrower: "Jane Smith".to_string(),
        interest_rate: "6.0".to_string(),
        duration: 24,
        review_status: ReviewStatus::Pending,
        creation_date: env.block.time.seconds(),
        approval_date: None,
        rejection_date: None,
    };

    // Store first loan
    let msg1 = ExecuteMsg::StoreLoan {
        user_id: "user1".to_string(),
        loan_id: "loan1".to_string(),
        loan: loan1.clone(),
    };
    execute(deps.as_mut(), env.clone(), info.clone(), msg1).unwrap();

    // Store second loan
    let msg2 = ExecuteMsg::StoreLoan {
        user_id: "user1".to_string(),
        loan_id: "loan2".to_string(),
        loan: loan2.clone(),
    };
    execute(deps.as_mut(), env.clone(), info, msg2).unwrap();

    // Query the loans for user1
    let res = query(deps.as_ref(), env, QueryMsg::GetLoansForUser {
        user_id: "user1".to_string(),
    }).unwrap();

    let queried_loans: Vec<LoanData> = from_json(&res).unwrap();
    assert_eq!(queried_loans.len(), 2);
    assert_eq!(queried_loans[0].amount, loan1.amount);
    assert_eq!(queried_loans[1].amount, loan2.amount);
}

#[test]
fn test_query_loan_statistics() {
    
    let mut deps = mock_dependencies();

    // Reviewer address
    let reviewer_addr:Addr = deps.api.addr_make("reviewer1");

    // Mock environment with block time for testing
    let env = mock_env();

    // Simulated current time for the test (using seconds)
    let current_time = env.block.time.seconds();

    // Store mock loan data for two months (current and last month)
    let month_seconds = 30 * 24 * 60 * 60; // Approx. one month in seconds
    let last_month_time = current_time - month_seconds;

    // Mock loans to be assigned to the reviewer
    //let reviewer_addr = Addr::unchecked("reviewer1");

    let loan1 = LoanData {
        amount: 100000,
        borrower: "John Doe".to_string(),
        interest_rate: "5.0".to_string(),
        duration: 12,
        review_status: ReviewStatus::Pending,  // Pending loan
        creation_date: current_time,           // Created this month
        approval_date: None,
        rejection_date: None,
    };

    let loan2 = LoanData {
        amount: 200000,
        borrower: "Jane Smith".to_string(),
        interest_rate: "6.0".to_string(),
        duration: 24,
        review_status: ReviewStatus::Approved,  // Approved loan
        creation_date: last_month_time,         // Created last month
        approval_date: Some(current_time),      // Approved this month
        rejection_date: None,
    };

    let loan3 = LoanData {
        amount: 150000,
        borrower: "Alice White".to_string(),
        interest_rate: "4.5".to_string(),
        duration: 6,
        review_status: ReviewStatus::Rejected,  // Rejected loan
        creation_date: last_month_time,         // Created last month
        approval_date: None,
        rejection_date: Some(last_month_time),  // Rejected last month
    };

    // Save the loans into storage
    LOAN_STORAGE.save(deps.as_mut().storage, (&"user1", &"loan1"), &loan1).unwrap();
    LOAN_STORAGE.save(deps.as_mut().storage, (&"user2", &"loan2"), &loan2).unwrap();
    LOAN_STORAGE.save(deps.as_mut().storage, (&"user3", &"loan3"), &loan3).unwrap();

    // Assign loans to reviewer
    REVIEWER_ASSIGNMENTS
        .save(deps.as_mut().storage, &reviewer_addr, &vec![
            ("user1".to_string(), "loan1".to_string()),
            ("user2".to_string(), "loan2".to_string()),
            ("user3".to_string(), "loan3".to_string())
        ]).unwrap();

        let msg = QueryMsg::GetLoanStatistics {
            reviewer: Some(reviewer_addr.to_string()),
        };
    // Query for statistics
    let res_bin = query(deps.as_ref(), env.clone(), msg).unwrap();

    // Deserialize the response
    let res: LoanStatistics = from_json(&res_bin).unwrap();

    // Validate pending loan count
    assert_eq!(res.pending_count, 1); // 1 loan is pending

    // Validate approved and rejected loans count
    assert_eq!(res.approved_this_month, 1); // 1 loan was approved this month
    assert_eq!(res.rejected_this_month, 0); // 0 loans were rejected this month
    assert_eq!(res.approved_last_month, 0); // 0 loans were approved last month
    assert_eq!(res.rejected_last_month, 1); // 1 loan was rejected last month

    // Validate average processing time
    assert_eq!(res.average_time_to_process.unwrap(), ((current_time - last_month_time) / 2).to_string());

    // Validate month-wise status count
    assert_eq!(res.month_wise_status_count.get(&format!("{}", current_time / month_seconds)).unwrap().get(&ReviewStatus::Pending.to_string()), Some(&1));
    assert_eq!(res.month_wise_status_count.get(&format!("{}", last_month_time / month_seconds)).unwrap().get(&ReviewStatus::Approved.to_string()), Some(&1));
    assert_eq!(res.month_wise_status_count.get(&format!("{}", last_month_time / month_seconds)).unwrap().get(&ReviewStatus::Rejected.to_string()), Some(&1));
}

#[test]
fn test_query_all_reviewer_statistics() {
    let mut deps = mock_dependencies();

    // Mock environment with block time for testing
    let env = mock_env();

    // Simulated current time for the test (using seconds)
    let current_time = env.block.time.seconds();
    let month_seconds = 30 * 24 * 60 * 60;
    let last_month_time = current_time - month_seconds;

    // Store mock loan data
    let reviewer1 = Addr::unchecked("reviewer1");
    let reviewer2 = Addr::unchecked("reviewer2");

    let loan1 = LoanData {
        amount: 100000,
        borrower: "John Doe".to_string(),
        interest_rate: "5.0".to_string(),
        duration: 12,
        review_status: ReviewStatus::Pending,
        creation_date: current_time,
        approval_date: None,
        rejection_date: None,
    };

    let loan2 = LoanData {
        amount: 200000,
        borrower: "Jane Smith".to_string(),
        interest_rate: "6.0".to_string(),
        duration: 24,
        review_status: ReviewStatus::Approved,
        creation_date: last_month_time,
        approval_date: Some(current_time),
        rejection_date: None,
    };

    // Save loans and assign to reviewers
    LOAN_STORAGE.save(deps.as_mut().storage, (&"user1", &"loan1"), &loan1).unwrap();
    LOAN_STORAGE.save(deps.as_mut().storage, (&"user2", &"loan2"), &loan2).unwrap();

    REVIEWER_ASSIGNMENTS.save(
        deps.as_mut().storage,
        &reviewer1,
        &vec![("user1".to_string(), "loan1".to_string())],
    ).unwrap();
    REVIEWER_ASSIGNMENTS.save(
        deps.as_mut().storage,
        &reviewer2,
        &vec![("user2".to_string(), "loan2".to_string())],
    ).unwrap();

    let msg = QueryMsg::GetAllReviewerStatistics {  };

    // Query statistics for all reviewers
    let res_bin = query(deps.as_ref(), env, msg).unwrap();

    // Deserialize the response
    let res: AllReviewerStatistics = from_json(&res_bin).unwrap();

    // Validate the statistics (example validation for brevity)
    assert_eq!(res.total_pending, 1);
    assert_eq!(res.total_approved, 1);
    assert_eq!(res.reviewers_stats.len(), 2);
}


}



