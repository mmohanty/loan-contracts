# Identity Smart Contract
[demo](https://www.youtube.com/watch?v=7M1xSdt7bNE)

## Overview

This smart contract allows users to create, update, and query identity metadata on the blockchain. It is built using CosmWasm and provides functionalities for managing digital identities in a decentralized manner.

A minimalistic web interface was developed to test the smart contract API. You can find it in [this repository](https://github.com/wotori/cw-dev-interface).

## Features

- **Mint/Update Identity**: Users can create a new identity by minting an identity metadata with one multipurpose UpdateMetadata method.
- **Query Identity**: Anyone can query the identity metadata of a specific user by their address.
- **Query All Identities**: Retrieve all identity metadata entries stored in the contract.

## Execute Messages

- Update Metadata:

```json
{
  "update_metadata": {
    "identity_data": {
      "name": "Alice Updated",
      "pic": "ipfs://newpic",
      "address": "cosmos1...",
      "about": "Updated About Alice",
      "avatar": "ipfs://newavatar"
    }
  }
}
```

## Query Messages

- User Info:

```json
{
  "user_info": {
    "address": "cosmos1..."
  }
}
```

`archway contracts query smart identity --args-file './queryMsg.json'`

- User Info All:

```json
{
  "user_info_all": {}
}
```

## Contract Structure

- lib.rs: Main entry point of the contract.
- exec.rs: Handles execution of contract functions such as minting and updating identities.
- models.rs: Defines the data structures used in the contract.
- msg.rs: Contains the message types for executing and querying the contract.
- que.rs: Handles query functions for the contract.
- states.rs: Manages the state storage for the contract.

## Deploy


https://docs.injective.network/cosmwasm-dapps/01_Cosmwasm_CW20_deployment_guide_Local.html


./setup.sh && injectived start


# inside the CosmWasm/cw-plus repo 
yes 12345678 | injectived tx wasm store target/wasm32-unknown-unknown/release/identity.wasm --from=genesis --chain-id="injective-1" --yes --fees=1000000000000000inj --gas=2000000




INIT='{}'
yes 12345678 | injectived tx wasm instantiate 1 $INIT --label="DecentralizedIdentity" --from=genesis --chain-id="injective-1" --yes --fees=1000000000000000inj --gas=2000000 --no-admin





injectived query tx Your_txhash
---
 key: code_id
    value: '"1"'
------

the address of the instantiated contract can be obtained on http://localhost:10337/swagger/#/Query/ContractsByCode


"contracts": [
    "inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz"
  ]

 -----



 the contract info meta data can be obtained on http://localhost:10337/swagger/#/Query/ContractInfo (opens new window)


 or by CLI query

CODE_ID=1
CONTRACT=$(injectived query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')
injectived query wasm contract $CONTRACT


injectived query tx Your_txhash

--
 key: contract_address
    value: '"inj14hj2tavq8fpesdwxxcu44rty3hh90vhujaxlnz"'
---



-- Query Data


BALANCE_QUERY='{"balance": {"address": "inj10cfy5e6qt2zy55q2w2ux2vuq862zcyf4fmfpj3"}}'
injectived query wasm contract-state smart $CONTRACT "$BALANCE_QUERY" --output json


IDENTITY_ALL_QUERY='{"user_info_all":{}}'
injectived query wasm contract-state smart $CONTRACT "$IDENTITY_ALL_QUERY" --output json


LOAN_ALL_QUERY='{"loan_data_all": {}}'
injectived query wasm contract-state smart $CONTRACT "$LOAN_ALL_QUERY" --output json


-- Execute command


LOAN_JSON='{"update_loandata":{"loan_data":{"loan_number":"123","loan_amount":"10000","interest_rate":"10","loan_duration":"10","loan_type":"AUTO","loan_status":"ACTIVE","loan_owner":"MANAS"}}}'

yes 12345678 | injectived tx wasm execute $CONTRACT "$LOAN_JSON" --from genesis --chain-id="injective-1" --yes --fees=1000000000000000inj --gas=2000000




--

IDENTITY_JSON='{"update_metadata": {"identity_data": {"name": "Alice Updated", "pic": "ipfs://newpic", "address": "cosmos1...", "about": "Updated About Alice", "avatar": "ipfs://newavatar" } } }'

yes 12345678 | injectived tx wasm execute $CONTRACT "$IDENTITY_JSON" --from genesis --chain-id="injective-1" --yes --fees=1000000000000000inj --gas=2000000







## License

This project is licensed under the MIT License
