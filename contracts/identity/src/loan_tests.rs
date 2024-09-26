mod test {

    use crate::models::{AllReviewerStatistics, FieldType, LoanData, LoanRequest, LoanStatistics, ReviewStatus};
    use crate::msg::{ExecuteMsg, QueryMsg};
    use crate::query::query;
    use crate::states::{LOAN_STORAGE, REVIEWER_ASSIGNMENTS, TEMPLATE_REVIEWERS, USER_TEMPLATES};

    use crate::exec::execute;
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::{attr, from_json, Addr};
    use std::collections::HashMap;

    #[test]
    fn test_create_loan_template() {
        let mut deps = mock_dependencies();

        let info = message_info(&Addr::unchecked("creator"), &[]);
        let env = mock_env();

        // Define the fields for the template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );
        fields.insert(
            "borrower".to_string(),
            FieldType::String {
                is_editable: true,
                format: None,
                min_value: None,
                max_value: None,
            },
        );
        fields.insert(
            "interest_rate".to_string(),
            FieldType::Number {
                is_editable: true,
                min_value: Some("0.5".to_string()),
                max_value: Some("5.0".to_string()),
            },
        );

        // Create the loan template
        let msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };
        let res = execute(deps.as_mut(), env, info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "create_loan_template"),
                attr("template_id", "template1"),
                attr("submitter", "creator"),
                attr("status", "pending"),
            ]
        );

        // Ensure the template is stored correctly under the user-specific key
        let stored_template = USER_TEMPLATES
            .load(&deps.storage, (&info.sender.to_string(), "template1"))
            .unwrap();
        assert_eq!(stored_template.name, "Home Loan");
        assert_eq!(stored_template.submitter, "creator");
        assert_eq!(stored_template.review_status, ReviewStatus::Pending);
    }

    #[test]
    fn test_submit_template_for_review() {
        let mut deps = mock_dependencies();

        let info = message_info(&Addr::unchecked("creator"), &[]);
        let env = mock_env();

        // Create the loan template first
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );
        fields.insert(
            "borrower".to_string(),
            FieldType::String {
                is_editable: true,
                format: None,
                min_value: None,
                max_value: None,
            },
        );

        // Create the loan template
        let msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Now submit the template for review
        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };
        let res = execute(deps.as_mut(), env, info, submit_msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "submit_template_for_review"),
                attr("template_id", "template1"),
                attr("reviewer", "reviewer1"),
                attr("status", "pending"),
            ]
        );

        // Ensure the reviewer assignment is stored correctly
        let reviewer = TEMPLATE_REVIEWERS.load(&deps.storage, "template1").unwrap();
        assert_eq!(reviewer.reviewer, "reviewer1");
    }

    #[test]
    fn test_approve_template() {
        let mut deps = mock_dependencies();

        let creator_info = message_info(&Addr::unchecked("creator"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let env = mock_env();

        // Create and submit the template for review
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();
        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };
        execute(deps.as_mut(), env.clone(), creator_info, submit_msg).unwrap();

        // Approve the template
        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };
        let res = execute(deps.as_mut(), env.clone(), reviewer_info, review_msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "review_template"),
                attr("template_id", "template1"),
                attr("status", "approved"),
            ]
        );

        // Ensure the template is now marked as approved
        let stored_template = USER_TEMPLATES
            .load(&deps.storage, (&"creator".to_string(), "template1"))
            .unwrap();
        assert_eq!(stored_template.review_status, ReviewStatus::Approved);
    }

    #[test]
    fn test_create_loan_with_approved_template() {
        let mut deps = mock_dependencies();

        let creator_info = message_info(&Addr::unchecked("creator"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let env = mock_env();

        // Create, submit, and approve the template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();
        execute(deps.as_mut(), env.clone(), creator_info.clone(), submit_msg).unwrap();
        execute(deps.as_mut(), env.clone(), reviewer_info, review_msg).unwrap();

        // Now create a loan based on the approved template
        let loan_request = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values
            },
        };

        let loan_create_msg = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request],
        };
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg,
        )
        .unwrap();
        assert_eq!(res.attributes[0].key, "created_loan_id");

        // Ensure the loan is stored correctly
        let loan_id = &res.attributes[0].value;
        let loan = LOAN_STORAGE
            .load(&deps.storage, (&"creator".to_string(), loan_id))
            .unwrap();
        assert_eq!(loan.template_id, "template1");
        assert_eq!(loan.values.get("amount").unwrap(), "60000");
    }

    #[test]
    fn test_assign_loans_to_reviewer() {
        let mut deps = mock_dependencies();

        // Reviewer address
        let verifier: Addr = deps.api.addr_make("reviewer1");

        let info = message_info(&Addr::unchecked("creator"), &[]);
        let msg = ExecuteMsg::AssignLoansToReviewer {
            reviewer: verifier.to_string(),
            loans: vec![
                ("user1".to_string(), "loan1".to_string()),
                ("user2".to_string(), "loan2".to_string()),
            ],
        };

        // Execute assign loans to reviewer
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // Assert response attributes
        assert_eq!(
            res.attributes,
            vec![
                attr("method", "assign_loans_to_reviewer"),
                attr("reviewer", verifier.to_string())
            ]
        );

        // Verify the loans have been assigned to the reviewer
        let assigned_loans = REVIEWER_ASSIGNMENTS.load(&deps.storage, &verifier).unwrap();
        assert_eq!(assigned_loans.len(), 2);
        assert_eq!(
            assigned_loans[0],
            ("user1".to_string(), "loan1".to_string())
        );
        assert_eq!(
            assigned_loans[1],
            ("user2".to_string(), "loan2".to_string())
        );
    }

    #[test]
    fn test_update_loan_review_status_to_approved() {
        let mut deps = mock_dependencies();

        let creator_info = message_info(&Addr::unchecked("creator"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let env = mock_env();

        // Step 1: Create the loan template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();

        // Step 2: Submit the template for review
        execute(deps.as_mut(), env.clone(), creator_info.clone(), submit_msg).unwrap();

        // Step 3: Approve the template
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer_info.clone(),
            review_msg,
        )
        .unwrap();

        // Step 4: Create a loan based on the approved template
        let loan_request = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values
            },
        };

        let loan_create_msg = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request],
        };

        let loan_res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg,
        )
        .unwrap();
        let loan_id = loan_res.attributes[0].value.clone();

        // Step 5: Update loan review status to Approved
        let loan_update_msg = ExecuteMsg::UpdateLoanReviewStatus {
            user_id: "creator".to_string(),
            loan_id: loan_id.clone(),
            new_status: ReviewStatus::Approved,
        };
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_update_msg,
        )
        .unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("method", "update_loan_review_status"),
                attr("user_id", "creator"),
                attr("loan_id", loan_id.clone())
            ]
        );

        // Step 6: Check the updated loan's status and approval date
        let loan = LOAN_STORAGE
            .load(&deps.storage, (&"creator".to_string(), &loan_id))
            .unwrap();
        assert_eq!(loan.review_status, ReviewStatus::Approved);
        assert!(loan.approval_date.is_some());
        assert!(loan.rejection_date.is_none()); // No rejection date when approved
    }

    #[test]
    fn test_update_loan_review_status_to_rejected() {
        let mut deps = mock_dependencies();

        let creator_info = message_info(&Addr::unchecked("creator"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let env = mock_env();

        // Step 1: Create the loan template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();

        // Step 2: Submit the template for review
        execute(deps.as_mut(), env.clone(), creator_info.clone(), submit_msg).unwrap();

        // Step 3: Approve the template
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer_info.clone(),
            review_msg,
        )
        .unwrap();

        // Step 4: Create a loan based on the approved template
        let loan_request = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values
            },
        };

        let loan_create_msg = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request],
        };

        let loan_res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg,
        )
        .unwrap();
        let loan_id = loan_res.attributes[0].value.clone();

        // Step 5: Update loan review status to Rejected
        let loan_update_msg = ExecuteMsg::UpdateLoanReviewStatus {
            user_id: "creator".to_string(),
            loan_id: loan_id.clone(),
            new_status: ReviewStatus::Rejected,
        };
        let res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_update_msg,
        )
        .unwrap();

        assert_eq!(
            res.attributes,
            vec![
                attr("method", "update_loan_review_status"),
                attr("user_id", "creator"),
                attr("loan_id", loan_id.clone())
            ]
        );

        // Step 6: Check the updated loan's status and rejection date
        let loan = LOAN_STORAGE
            .load(&deps.storage, (&"creator".to_string(), &loan_id))
            .unwrap();
        assert_eq!(loan.review_status, ReviewStatus::Rejected);
        assert!(loan.rejection_date.is_some());
        assert!(loan.approval_date.is_none()); // No approval date when rejected
    }

    #[test]
    fn test_query_loan() {
        let mut deps = mock_dependencies();

        let creator_info = message_info(&Addr::unchecked("creator"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let env = mock_env();

        // Step 1: Create the loan template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );
        fields.insert(
            "borrower".to_string(),
            FieldType::String {
                is_editable: true,
                format: None,
                min_value: None,
                max_value: None,
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();

        // Step 2: Submit the template for review
        execute(deps.as_mut(), env.clone(), creator_info.clone(), submit_msg).unwrap();

        // Step 3: Approve the template
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer_info.clone(),
            review_msg,
        )
        .unwrap();

        // Step 4: Create a loan based on the approved template
        let loan_request = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values.insert("borrower".to_string(), "John Doe".to_string());
                values
            },
        };

        let loan_create_msg = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request],
        };

        let loan_res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg,
        )
        .unwrap();
        let loan_id = loan_res.attributes[0].value.clone();

        // Step 5: Query the loan
        let loan_query_msg = QueryMsg::GetLoanDetails {
            user_id: "creator".to_string(),
            loan_id: loan_id.clone(),
        };
        let loan_bin = query(deps.as_ref(), env.clone(), loan_query_msg).unwrap();

        //Deserialize the loan
        let loan: LoanData = from_json(&loan_bin).unwrap();

        // Step 6: Validate the queried loan
        assert_eq!(loan.loan_id, loan_id);
        assert_eq!(loan.template_id, "template1");
        assert_eq!(loan.values.get("amount").unwrap(), "60000");
        assert_eq!(loan.values.get("borrower").unwrap(), "John Doe");
        assert_eq!(loan.review_status, ReviewStatus::Pending); // Since it's not approved/rejected yet
        assert!(loan.approval_date.is_none());
        assert!(loan.rejection_date.is_none());
    }

    #[test]
    fn test_query_nonexistent_loan() {
        let deps = mock_dependencies();

        // Try to query a loan that doesn't exist
        let loan_query_msg = QueryMsg::GetLoanDetails {
            user_id: "creator".to_string(),
            loan_id: "nonexistent_loan".to_string(),
        };
        let res_bin = query(deps.as_ref(), mock_env().clone(), loan_query_msg);

        // Ensure an error is returned
        assert!(res_bin.is_err());
    }

    #[test]
    fn test_query_loans_for_user() {
        let mut deps = mock_dependencies();

        let creator_info = message_info(&Addr::unchecked("creator1"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let env = mock_env();

        // Step 1: Create the loan template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );
        fields.insert(
            "borrower".to_string(),
            FieldType::String {
                is_editable: true,
                format: None,
                min_value: None,
                max_value: None,
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields,
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();

        // Step 2: Submit the template for review
        execute(deps.as_mut(), env.clone(), creator_info.clone(), submit_msg).unwrap();

        // Step 3: Approve the template
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer_info.clone(),
            review_msg,
        )
        .unwrap();

        // Step 4: Create multiple loans based on the approved template
        let loan_request_1 = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values.insert("borrower".to_string(), "John Doe".to_string());
                values
            },
        };
        let loan_request_2 = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "80000".to_string());
                values.insert("borrower".to_string(), "Jane Smith".to_string());
                values
            },
        };

        // Create the two loans
        let loan_create_msg1 = ExecuteMsg::CreateLoan {
            user_id: "creator1".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };

        let loan_create_msg2 = ExecuteMsg::CreateLoan {
            user_id: "creator1".to_string(),
            loan_requests: vec![loan_request_2.clone()],
        };
        execute(deps.as_mut(),env.clone(),creator_info.clone(), loan_create_msg1,).unwrap();
        execute(deps.as_mut(),env.clone(),creator_info.clone(),loan_create_msg2,).unwrap();

        // Step 5: Query all loans for the user
        let loan_query_msg = QueryMsg::GetLoansForUser {
            user_id: "creator1".to_string(),
        };
        let loans_bin = query(deps.as_ref(), mock_env().clone(), loan_query_msg).unwrap();

        //Deserialize the loans
        let loans: Vec<LoanData> = from_json(&loans_bin).unwrap();

        // Step 6: Validate the loans
        assert_eq!(loans.len(), 2);

        // Check the first loan
        let loan_1 = &loans[0];
        assert_eq!(loan_1.template_id, "template1");
        assert_eq!(true, loan_1.values.get("amount").unwrap() == "60000" || loan_1.values.get("amount").unwrap() == "80000");
        assert_eq!(true, loan_1.values.get("borrower").unwrap()== "Jane Smith" || loan_1.values.get("borrower").unwrap() == "John Doe");

        // Check the second loan
        let loan_2 = &loans[1];
        assert_eq!(loan_2.template_id, "template1");
        assert_eq!(true, loan_1.values.get("amount").unwrap() == "60000" || loan_1.values.get("amount").unwrap() == "80000");
        assert_eq!(true, loan_1.values.get("borrower").unwrap()== "Jane Smith" || loan_1.values.get("borrower").unwrap() == "John Doe");
    }

    #[test]
    fn test_query_loans_for_user_no_loans() {
        let deps = mock_dependencies();

        // Query a user with no loans
        let loan_query_msg = QueryMsg::GetLoansForUser {
            user_id: "creator".to_string(),
        };
        let loans = query(deps.as_ref(), mock_env().clone(), loan_query_msg).unwrap();

        // Deserialize the loans
        let loans: Vec<LoanData> = from_json(&loans).unwrap();

        // Ensure the result is an empty vector
        assert!(loans.is_empty());
    }
    #[test]
    fn test_query_loan_statistics() {
        let mut deps = mock_dependencies();
        let creator_info = message_info(&Addr::unchecked("creator"), &[]);
        let reviewer_info = message_info(&Addr::unchecked("reviewer1"), &[]);

        let env = mock_env();

        // Step 1: Create loan template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );
        fields.insert(
            "borrower".to_string(),
            FieldType::String {
                is_editable: false,
                format: None,
                min_value: None,
                max_value: None,
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields: fields.clone(),
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(deps.as_mut(), env.clone(), creator_info.clone(), create_msg).unwrap();

        // Step 2: Submit and approve the template
        execute(deps.as_mut(), env.clone(), creator_info.clone(), submit_msg).unwrap();
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer_info.clone(),
            review_msg,
        )
        .unwrap();

        // Step 3: Create three loans with different statuses
        let loan_request_1 = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values.insert("borrower".to_string(), "John Doe".to_string());
                values
            },
        };

        let loan_create_msg1 = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };

        // Loan 1: Pending
        let loan_res1 = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg1,
        )
        .unwrap();

        // Loan 2:
        let loan_create_msg2 = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };
        let loan_res2 = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg2,
        )
        .unwrap();

        // Loan 3:
        let loan_create_msg3 = ExecuteMsg::CreateLoan {
            user_id: "creator".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };
        let loan_res3 = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_create_msg3,
        )
        .unwrap();

        // assign loans to reviewer
        let verifier: Addr = deps.api.addr_make("reviewer1");
        let assign_to_reviewer_msg = ExecuteMsg::AssignLoansToReviewer {
            reviewer: verifier.to_string(),
            loans: vec![
                (
                    "creator".to_string(),
                    loan_res1.attributes[0].value.to_string(),
                ),
                (
                    "creator".to_string(),
                    loan_res2.attributes[0].value.to_string(),
                ),
                (
                    "creator".to_string(),
                    loan_res3.attributes[0].value.to_string(),
                ),
            ],
        };

        let _res = execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            assign_to_reviewer_msg,
        )
        .unwrap();

        // Loan 2: Approved
        let loan_update_msg2 = ExecuteMsg::UpdateLoanReviewStatus {
            user_id: "creator".to_string(),
            loan_id: loan_res2.attributes[0].value.clone(),
            new_status: ReviewStatus::Approved,
        };
        execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_update_msg2,
        )
        .unwrap();

        // Loan 3: Rejected
        let loan_update_msg3 = ExecuteMsg::UpdateLoanReviewStatus {
            user_id: "creator".to_string(),
            loan_id: loan_res3.attributes[0].value.clone(),
            new_status: ReviewStatus::Rejected,
        };
        execute(
            deps.as_mut(),
            env.clone(),
            creator_info.clone(),
            loan_update_msg3,
        )
        .unwrap();

        // Step 4: Query loan statistics for the user
        // Reviewer address
        let verifier: Addr = deps.api.addr_make("reviewer1");
        let loan_query_msg = QueryMsg::GetLoanStatistics {
            reviewer: Some(verifier.to_string()),
        };
        let stats_bin = query(deps.as_ref(), mock_env().clone(), loan_query_msg).unwrap();

        let stats: LoanStatistics = from_json(&stats_bin).unwrap();

        // Step 5: Validate statistics
        assert_eq!(stats.rejected_this_month, 1);
        assert_eq!(stats.approved_this_month, 1);
        assert_eq!(stats.pending_count, 1);
    }

    #[test]
    fn test_query_all_reviewer_loan_statistics() {
        let mut deps = mock_dependencies();
        let creator1_info = message_info(&Addr::unchecked("creator1"), &[]);
        let creator2_info = message_info(&Addr::unchecked("creator2"), &[]);
        let reviewer1_info = message_info(&Addr::unchecked("reviewer1"), &[]);
        let reviewer2_info = message_info(&Addr::unchecked("reviewer2"), &[]);

        let env = mock_env();

        // Step 1: Create loan template
        let mut fields = HashMap::new();
        fields.insert(
            "amount".to_string(),
            FieldType::Number {
                is_editable: false,
                min_value: Some("50000.0".to_string()),
                max_value: Some("1000000.0".to_string()),
            },
        );

        fields.insert(
            "borrower".to_string(),
            FieldType::String {
                is_editable: false,
                format: None,
                min_value: None,
                max_value: None,
            },
        );

        let create_msg = ExecuteMsg::CreateLoanTemplate {
            template_id: "template1".to_string(),
            name: "Home Loan".to_string(),
            fields: fields.clone(),
        };

        let submit_msg = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template1".to_string(),
            reviewer: "reviewer1".to_string(),
        };

        let review_msg = ExecuteMsg::ReviewTemplate {
            template_id: "template1".to_string(),
            approve: true,
        };

        execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            create_msg,
        )
        .unwrap();

        // Step 2: Submit and approve the template
        execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            submit_msg,
        )
        .unwrap();
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer1_info.clone(),
            review_msg,
        )
        .unwrap();

        //Template 2

        let mut fields2 = fields.clone();

        fields2.insert(
            "borrower2".to_string(),
            FieldType::String {
                is_editable: false,
                format: None,
                min_value: None,
                max_value: None,
            },
        );

        let create_template_msg1 = ExecuteMsg::CreateLoanTemplate {
            template_id: "template2".to_string(),
            name: "Home Loan 2".to_string(),
            fields: fields2.clone(),
        };

        let submit_template_msg1 = ExecuteMsg::SubmitTemplateForReview {
            template_id: "template2".to_string(),
            reviewer: "reviewer2".to_string(),
        };

        let review_template_msg1 = ExecuteMsg::ReviewTemplate {
            template_id: "template2".to_string(),
            approve: true,
        };

        execute(
            deps.as_mut(),
            env.clone(),
            creator2_info.clone(),
            create_template_msg1,
        )
        .unwrap();

        // Step 2: Submit and approve the template
        execute(
            deps.as_mut(),
            env.clone(),
            creator2_info.clone(),
            submit_template_msg1,
        )
        .unwrap();
        execute(
            deps.as_mut(),
            env.clone(),
            reviewer2_info.clone(),
            review_template_msg1,
        )
        .unwrap();

        // Step 3: Create 4 loans with different statuses and different reviewer
        let loan_request_1 = LoanRequest {
            template_id: "template1".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values.insert("borrower".to_string(), "John Doe".to_string());
                values
            },
        };

        let loan_create_msg1 = ExecuteMsg::CreateLoan {
            user_id: "creator1".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };

        // Loan 1: Pending
        let loan_res1 = execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            loan_create_msg1,
        )
        .unwrap();

        // Loan 2:
        let loan_create_msg2 = ExecuteMsg::CreateLoan {
            user_id: "creator1".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };
        let loan_res2 = execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            loan_create_msg2,
        )
        .unwrap();

        // Loan 3:
        let loan_create_msg3 = ExecuteMsg::CreateLoan {
            user_id: "creator1".to_string(),
            loan_requests: vec![loan_request_1.clone()],
        };
        let loan_res3 = execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            loan_create_msg3,
        )
        .unwrap();

        // assign loans to reviewer
        let verifier: Addr = deps.api.addr_make("reviewer1");
        let assign_to_reviewer_msg = ExecuteMsg::AssignLoansToReviewer {
            reviewer: verifier.to_string(),
            loans: vec![
                (
                    "creator1".to_string(),
                    loan_res1.attributes[0].value.to_string(),
                ),
                (
                    "creator1".to_string(),
                    loan_res2.attributes[0].value.to_string(),
                ),
                (
                    "creator1".to_string(),
                    loan_res3.attributes[0].value.to_string(),
                ),
            ],
        };

        let _res = execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            assign_to_reviewer_msg,
        )
        .unwrap();

        // Loan 2: Approved
        let loan_update_msg2 = ExecuteMsg::UpdateLoanReviewStatus {
            user_id: "creator1".to_string(),
            loan_id: loan_res2.attributes[0].value.clone(),
            new_status: ReviewStatus::Approved,
        };
        execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            loan_update_msg2,
        )
        .unwrap();

        // Loan 3: Rejected
        let loan_update_msg3 = ExecuteMsg::UpdateLoanReviewStatus {
            user_id: "creator1".to_string(),
            loan_id: loan_res3.attributes[0].value.clone(),
            new_status: ReviewStatus::Rejected,
        };
        execute(
            deps.as_mut(),
            env.clone(),
            creator1_info.clone(),
            loan_update_msg3,
        )
        .unwrap();

        //Loan 4 : Different reviewer and creator
        // Loan 4:
        let loan_request_4 = LoanRequest {
            template_id: "template2".to_string(),
            values: {
                let mut values = HashMap::new();
                values.insert("amount".to_string(), "60000".to_string());
                values.insert("borrower".to_string(), "John Doe".to_string());
                values.insert("borrower2".to_string(), "Frank Doe".to_string());
                values
            },
        };
        let loan_create_msg4 = ExecuteMsg::CreateLoan {
            user_id: "creator2".to_string(),
            loan_requests: vec![loan_request_4.clone()],
        };
        let loan_res4 = execute(
            deps.as_mut(),
            env.clone(),
            creator2_info.clone(),
            loan_create_msg4,
        )
        .unwrap();

        // assign loan 4 to reviewer
        let verifier2: Addr = deps.api.addr_make("reviewer2");
        let assign_to_reviewer2_msg1 = ExecuteMsg::AssignLoansToReviewer {
            reviewer: verifier2.to_string(),
            loans: vec![(
                "creator2".to_string(),
                loan_res4.attributes[0].value.to_string(),
            )],
        };

        execute(
            deps.as_mut(),
            env.clone(),
            creator2_info.clone(),
            assign_to_reviewer2_msg1,
        )
        .unwrap();

        // Step 4: Query loan statistics for the user
        // Reviewer address
        let loan_query_msg = QueryMsg::GetAllReviewerStatistics{
        };
        let stats_bin = query(deps.as_ref(), mock_env().clone(), loan_query_msg).unwrap();

        let stats: AllReviewerStatistics = from_json(&stats_bin).unwrap();

        // Step 5: Validate statistics
        assert_eq!(stats.total_pending, 2);
        assert_eq!(stats.total_approved, 1);
        assert_eq!(stats.total_rejected, 1);
    }
}
