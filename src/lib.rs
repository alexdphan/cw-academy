use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use msg::InstantiateMsg;


mod contract; // private because contract contains internal logic functions, contains all msg handlers 
pub mod msg; // using module file msg.rs
mod state;

// In the context of a smart contract, an entry point is a function that can be called by external users or other contracts. Entry points serve as the public interface for a contract, allowing external entities to interact with it and execute its functions.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, msg.counter)
} 

#[entry_point]
pub fn execute(
    deps: DepsMut,
     _env: Env, 
     info: MessageInfo, 
     msg: msg::ExecMsg,
    ) -> StdResult<Response> {
    use contract::exec;
    use msg::ExecMsg::*;

    match msg {
        Poke {} => exec::poke(deps, info),
    }
} // align the call of the exec::poke in the entry point:


// Deps is read-only, DepsMut is read-write on blockchain state
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&query::value(deps)?),
    }
}
// returns binary data (JSON serialized responses instead of Response (for Query Messages)
// pub fn can be called from anywhere
// fn can only be called from within the current module



// -------------- TESTS -------------- // 
// tests can be in another file, but we can also put them here in lib crate
// cfg is a conditional compilation attribute, meaning that the code it wraps would be compiled-in if and only if the predicate passed to it is true. In this case, we have test predicate, which is true on cargo test runs
#[cfg(test)]
mod test {
    use cosmwasm_std::{Empty, Addr}; // msg type for contract
    use cw_multi_test::{App, Contract, ContractWrapper, Executor}; 
    // App: simulate the execution of a smart contract in a controlled environment
    // Contract: simulates a contract, testing
    // ContractWrapper: wrapper for contract (implements 'Contract' trait), used for testing

      use crate::msg::{InstantiateMsg, QueryMsg, ValueResp, ExecMsg};
    use crate::{execute, instantiate, query}; // brings the execute, instantiate, and query functions from the current crate into the current scope

    // https://dhghomon.github.io/easy_rust/Chapter_54.html
   fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
    // Ihe counting_contract function returns a Box containing the ContractWrapper instance as a trait object. 
    // In this case, the counting_contract function creates an instance of a struct called ContractWrapper and passes it three functions as arguments: execute, instantiate, and query. 
    // These functions are likely implementations of the Contract trait's methods, and they will be used to execute the contract's behavior when the trait object is called.

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
        // simulates the instantiation of a contract, gives back a contract address
        .instantiate_contract( 
                contract_id, // contract id the contract was deployed with
                Addr::unchecked("sender"), // create an arbitrary address, used for testing
                &InstantiateMsg { counter: 10 }, // init msg, msg you send to contract when you instantiate it
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

        assert_eq!(resp, ValueResp { value: 10 });
    }

    #[test]
    fn poke() {
        let mut app = App::default(); 
        // app is a testing framework

        let contract_id = app.store_code(counting_contract()); 
        // simulates the deployment of a contract, gives back a contract id

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: 0 },
                &[],
                "Counting Contract",
                None,
            ).unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Poke {},
           &[],
        ).unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }
}