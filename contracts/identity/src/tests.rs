#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::Addr;

    use crate::models::{IdentityMetadata, LoanData};
    use crate::states::IDENTITIES;
    use crate::{exec, query};

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

        let res = exec::upsert_identity(deps.as_mut(), env.clone(), info.clone(), metadata.clone())
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

        let _ = exec::upsert_identity(deps.as_mut(), env.clone(), info.clone(), metadata.clone())
            .unwrap();

        let updated_metadata = IdentityMetadata {
            name: "Alice Updated".to_string(),
            pic: "ipfs://newpic".to_string(),
            address: Addr::unchecked("cosmos1...".to_string()),
            about: "Updated About Alice".to_string(),
            avatar: "ipfs://newavatar".to_string(),
        };

        let res = exec::upsert_identity(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            updated_metadata.clone(),
        )
        .unwrap();
        assert_eq!(res.attributes[0].value, "update_metadata");

        let stored_metadata = IDENTITIES.load(&deps.storage, &info.sender).unwrap();
        assert_eq!(stored_metadata, updated_metadata);
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

        let _ = exec::upsert_identity(deps.as_mut(), env.clone(), info.clone(), metadata.clone())
            .unwrap();

        let res = query::query_identity(deps.as_ref(), info.sender.clone()).unwrap();

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

        let _ = exec::upsert_identity(deps.as_mut(), env.clone(), info1.clone(), metadata1.clone())
            .unwrap();
        let _ = exec::upsert_identity(deps.as_mut(), env.clone(), info2.clone(), metadata2.clone())
            .unwrap();

        let res = query::query_all_identities(deps.as_ref()).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], (info1.sender.clone(), metadata1));
        assert_eq!(res[1], (info2.sender.clone(), metadata2));
    }

    #[test]
    fn test_query_all_loans() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        

        let info1 = message_info(&Addr::unchecked("creator1".to_string()), &[]);
        let info2 = message_info(&Addr::unchecked("creator2".to_string()), &[]);

        let metadata1 = LoanData {
            loan_number: "123".to_string(),
            loan_amount: "10000".to_string(),
            loan_duration: "5".to_string(),
            loan_status: "Active".to_string(),
            loan_owner: "manas".to_string(),
            loan_type: "AUTO".to_string(),
            interest_rate: "10".to_string(),
        };

        let metadata2 = LoanData {
            loan_number: "234".to_string(),
            loan_amount: "20000".to_string(),
            loan_duration: "5".to_string(),
            loan_status: "Active".to_string(),
            loan_owner: "mohanty".to_string(),
            loan_type: "AUTO".to_string(),
            interest_rate: "10".to_string(),
        };

        let _ = exec::upsert_loan(deps.as_mut(), env.clone(), info1.clone(), metadata1.clone())
            .unwrap();
        let _ = exec::upsert_loan(deps.as_mut(), env.clone(), info2.clone(), metadata2.clone())
            .unwrap();

        let res = query::query_all_loans(deps.as_ref()).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0], ("1234".to_owned(), metadata1));
        assert_eq!(res[1], ("234".to_owned(), metadata2));
    }

}
