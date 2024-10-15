#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::exec::execute;
    use crate::models::LoanStatistics;
    use crate::msg::{ExecuteMsg, QueryMsg};
    use crate::query::query;

    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env};
    use cosmwasm_std::{coins, from_json, Addr, DepsMut, Env, MessageInfo};


    //Setup for Tests
    fn mock_info(sender: &str, funds: &[u128]) -> MessageInfo {
        MessageInfo {
            sender: Addr::unchecked(sender),
            funds: coins(funds[0], "ucosm"),
        }
    }

    fn setup_basic_template(deps: DepsMut, env: Env, creator: &str, reviewer: &str) {
        let msg = ExecuteMsg::CreateTemplate {
            id: "template1".to_string(),
            name: "Loan Template 1".to_string(),
            loan_creators: vec![creator.to_string()],
            loan_reviewers: vec![reviewer.to_string()],
            fields: HashMap::new(),
        };

        let info = mock_info(creator, &[100]);
        execute(deps, env, info, msg).unwrap();
    }


    fn setup_basic_loan(deps: DepsMut, env: Env, creator: &str) {
        let msg = ExecuteMsg::CreateLoan {
            id: "loan1".to_string(),
            template_id: "template1".to_string(),
            attributes: HashMap::new(),
        };

        let info = mock_info(creator, &[100]);
        execute(deps, env, info, msg).unwrap();
    }


    //Test Case 1: Create a Template

    #[test]
    fn test_create_template() {
        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::CreateTemplate {
            id: "template1".to_string(),
            name: "Loan Template 1".to_string(),
            loan_creators: vec!["creator1".to_string()],
            loan_reviewers: vec!["reviewer1".to_string()],
            fields: HashMap::new(),
        };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "create_template");
    }


    //Test Case 2: Submit a Template
    #[test]
    fn test_submit_template() {

        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        setup_basic_template(deps.as_mut(), env.clone(), "creator1", "reviewer1");

        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitTemplate { id: "template1".to_string() };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "submit_template");
    }


    //Test Case 3: Approve Template 
    #[test]
    fn test_approve_template() {

        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        setup_basic_template(deps.as_mut(), env.clone(), "creator1","reviewer1");
        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitTemplate { id: "template1".to_string() };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve as creator
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: true,
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(res.attributes[0].value, "approve_template");

        // Approve as reviewer
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: false,
        };

        let reviewer_info = mock_info("reviewer1", &[100]);
        let res = execute(deps.as_mut(), env, reviewer_info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "approve_template");
    }

    //Test Case 4: Create a Loan

    #[test]
    fn test_create_loan() {

        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        let msg = ExecuteMsg::CreateTemplate {
            id: "template1".to_string(),
            name: "Loan Template 1".to_string(),
            loan_creators: vec!["creator1".to_string()],
            loan_reviewers: vec!["reviewer1".to_string()],
            fields: HashMap::new(),
        };

        let info = mock_info("creator1", &[100]);
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitTemplate { id: "template1".to_string() };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve as creator
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: true,
        };

       execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve as reviewer
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: false,
        };

        let reviewer_info = mock_info("reviewer1", &[100]);
        execute(deps.as_mut(), env.clone(), reviewer_info, msg).unwrap();

        let msg = ExecuteMsg::CreateLoan {
            id: "loan1".to_string(),
            template_id: "template1".to_string(),
            attributes: HashMap::new(),
        };

        

        let info = mock_info("creator1", &[100]);
        let res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "create_loan");
    }

    //Test Case 5: Submit a Loan

    #[test]
    fn test_submit_loan() {

        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        let msg = ExecuteMsg::CreateTemplate {
            id: "template1".to_string(),
            name: "Loan Template 1".to_string(),
            loan_creators: vec!["creator1".to_string()],
            loan_reviewers: vec!["reviewer1".to_string()],
            fields: HashMap::new(),
        };

        let info = mock_info("creator1", &[100]);
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitTemplate { id: "template1".to_string() };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve as creator
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: true,
        };

       execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve as reviewer
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: false,
        };

        let reviewer_info = mock_info("reviewer1", &[100]);
        execute(deps.as_mut(), env.clone(), reviewer_info, msg).unwrap();


        setup_basic_loan(deps.as_mut(), env.clone(), "creator1");

        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitLoan { loan_id: "loan1".to_string() };

        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(res.attributes[0].value, "submit_loan");
    }

    //Test Case 6: Approve a Loan
    #[test]
    fn test_approve_loan() {

        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        // Create Template
        let msg = ExecuteMsg::CreateTemplate {
            id: "template1".to_string(),
            name: "Loan Template 1".to_string(),
            loan_creators: vec!["creator1".to_string()],
            loan_reviewers: vec!["reviewer1".to_string()],
            fields: HashMap::new(),
        };

        let info = mock_info("creator1", &[100]);
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitTemplate { id: "template1".to_string() };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve template as creator
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: true,
        };

       execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve template as reviewer
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: false,
        };

        let reviewer_info = mock_info("reviewer1", &[100]);
        execute(deps.as_mut(), env.clone(), reviewer_info, msg).unwrap();

        // create loan
        let msg = ExecuteMsg::CreateLoan {
            id: "loan1".to_string(),
            template_id: "template1".to_string(),
            attributes: HashMap::new(),
        };

        let info = mock_info("creator1", &[100]);
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // submit loan
        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitLoan { loan_id: "loan1".to_string() };

        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        //Review loan with Status approved
        let info_reviewer = mock_info("reviewer1", &[100]);
        let msg = ExecuteMsg::ReviewLoan {
            loan_id: "loan1".to_string(),
            approve: true,
            return_loan: false,
            comments: Some("Approved.".to_string()),
        };

        let res = execute(deps.as_mut(), env, info_reviewer, msg).unwrap();
        assert_eq!(res.attributes[0].value, "review_loan");
        assert_eq!(res.attributes[1].value, "loan1");
        assert_eq!(res.attributes[2].value, "approved");
    }


    //Test Case 7: Query Loan Statistics (with processing time)

    #[test]
    fn test_query_loan_statistics() {

        let mut deps = mock_dependencies_with_balance(&[]);
        let env = mock_env();

        // Create Template
        let msg = ExecuteMsg::CreateTemplate {
            id: "template1".to_string(),
            name: "Loan Template 1".to_string(),
            loan_creators: vec!["creator1".to_string()],
            loan_reviewers: vec!["reviewer1".to_string()],
            fields: HashMap::new(),
        };

        let info = mock_info("creator1", &[100]);
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Submit Template
        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitTemplate { id: "template1".to_string() };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve template as creator
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: true,
        };

       execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Approve template as reviewer
        let msg = ExecuteMsg::ApproveTemplate {
            id: "template1".to_string(),
            as_creator: false,
        };

        let reviewer_info = mock_info("reviewer1", &[100]);
        execute(deps.as_mut(), env.clone(), reviewer_info, msg).unwrap();

        // create loan
        setup_basic_loan(deps.as_mut(), env.clone(), "creator1");

        // submit loan
        let info = mock_info("creator1", &[100]);
        let msg = ExecuteMsg::SubmitLoan { loan_id: "loan1".to_string() };
        execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        //Review loan with Status approved
        let info_reviewer = mock_info("reviewer1", &[100]);
        let msg = ExecuteMsg::ReviewLoan{
            loan_id: "loan1".to_string(),
            approve: true,
            return_loan: false,
            comments: Some("Approved.".to_string()),
        };
        execute(deps.as_mut(), env.clone(), info_reviewer.clone(), msg).unwrap();

        // Query statistics
        let msg = QueryMsg::QueryLoanStatistics {};
        let res = query(deps.as_ref(), env, msg).unwrap();
        let stats: LoanStatistics = from_json(&res).unwrap();

        assert_eq!(stats.total_loans, 1);
        assert_eq!(stats.approved_count, 1);
        assert!(stats.avg_processing_time.is_some());
    }

}
