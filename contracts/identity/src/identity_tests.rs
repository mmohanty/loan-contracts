#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{message_info, mock_dependencies, mock_env};
    use cosmwasm_std::{ from_json, Addr};

    use crate::exec::execute;
    use crate::models::IdentityMetadata;
    use crate::msg::{ExecuteMsg, QueryMsg};
    use crate::query::query;
    use crate::states::IDENTITIES;

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

}



