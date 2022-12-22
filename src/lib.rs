use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
mod contract; // private because contract contains internal logic functions, contains all msg handlers 
pub mod msg; // using module file msg.rs

// In the context of a smart contract, an entry point is a function that can be called by external users or other contracts. Entry points serve as the public interface for a contract, allowing external entities to interact with it and execute its functions.
#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    // Ok(Response::default())
    Ok(Response::new())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

// Deps is read-only, DepsMut is read-write on blockchain state
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
       Value {} => to_binary(&query::value()),
       Incremented { value } => to_binary(&query::incremented(value)),
    }
}
// returns binary data (JSON serialized responses instead of Response (for Query Messages)
// pub fn can be called from anywhere
// fn can only be called from within the current module

// tests can be in another file, but we can also put them here in lib crate
// cfg is a conditional compilation attribute, meaning that the code it wraps would be compiled-in if and only if the predicate passed to it is true. In this case, we have test predicate, which is true on cargo test runs
#[cfg(test)]
mod test {
    use cosmwasm_std::{Empty, Addr}; // msg type for contract
    use cw_multi_test::{App, Contract, ContractWrapper, Executor}; 
    // App: simulate the execution of a smart contract in a controlled environment
    // Contract: simulates a contract, testing
    // ContractWrapper: wrapper for contract (implements 'Contract' trait), used for testing

    use crate::{execute, instantiate, query, msg::{QueryMsg, ValueResp}}; // brings the execute, instantiate, and query functions from the current crate into the current scope

    // https://dhghomon.github.io/easy_rust/Chapter_54.html
    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        let mut app = App::default(); // App instance is the soul of the testing framework, it is the blockchain simulator, and it would be an interface to all contracts on it.
        
        let contract_id = app.store_code(counting_contract());
        // simulates the deployment of a contract, gives back a contract id
        // We use it right after it is created to call store_code on it. 
        // This method takes a contract as an argument and returns a contract id (unique identifier for the contract)
        // key-value store, contract id is the key, contract is the value
        // Storing code is this operation of uploading smart contract binary to be stored in a blockchain state.

        let contract_addr = app // app is a testing framework
        .instantiate_contract( // simulates the instantiation of a contract, gives back a contract address
                contract_id, // contract id the contract was deployed with
                Addr::unchecked("sender"), // create an arbitrary address, used for testing
                &Empty {}, // init msg, msg you send to contract when you instantiate it
                // but what is important - is it would not be just passed to the entry point. It would be first serialized to JSON and then deserialized back to send it to the contract.
                &[], // funds, usually has tokens sent with the contract instantiation
                "Counting Contract", // label, name of the contract
                None, // admins are the only addresses that can later perform migrations of smart contracts.
            ).unwrap(); // fail if error
        // simulates the instantiation of a contract, gives back a contract address
        
        let resp: ValueResp = app // app is a testing framework
        .wrap() // converts app, wrap the contract to a temporary QueryWrapper object, allowing to query the query the blockchain
        .query_wasm_smart(contract_addr, &QueryMsg::Value {}) // query the contract with the contract address and the query message, depending who calls, they can't mod the state
        .unwrap(); // fail if error

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn query_incremented() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app.instantiate_contract(
            contract_id,
        Addr::unchecked("sender"),
        &Empty {},
        &[],
        "Counting Contract",
        None,
        ).unwrap();

        let resp: ValueResp = app.wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Incremented { value: 1 }).unwrap();

        assert_eq!(resp, ValueResp { value: 2 });
    }
}