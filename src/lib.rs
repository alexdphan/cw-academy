use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
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
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg.counter, msg.minimal_donation) 
} // entry point instantiate function for contract.rs

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> StdResult<Response> {
    use contract::exec;
    use msg::ExecMsg::*;

  match msg {
        Donate {} => exec::donate(deps, info),
        Reset { counter } => exec::reset(deps, info, counter),
        Withdraw {} => exec::withdraw(deps, env, info),
    }
} 


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
    use cosmwasm_std::{Empty, Addr, coin, coins, Storage}; // msg type for contract
    use cw_multi_test::{App, Contract, ContractWrapper, Executor, Router}; 
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
    fn query_value() { // test to see if the query_value function works
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
                &InstantiateMsg { counter: 10, minimal_donation: coin(10, "atom") }, // init msg, msg you send to contract when you instantiate it
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
    fn reset() { // test to see if the contract (counter) can be reset
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: 0, minimal_donation: coin(10, "atom") },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Reset { counter: 10 },
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 10 });
    }

    #[test]
    fn donate() { // test to see if the contract can be donated to
let mut app = App::default(); 

let contract_id = app.store_code(counting_contract());

let contract_addr = app
    .instantiate_contract(
        contract_id,
        Addr::unchecked("sender"),
        &InstantiateMsg { 
            counter: 0, 
            minimal_donation: coin(10, "atom") },
        &[],
        "Counting contract",
        None,
    ).unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecMsg::Donate {},
        &[],
    ).unwrap();
    
    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Value {})
        .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });

    }

    #[test]
fn donate_with_funds() { // test to check if the contract is able to receive funds

    let sender = Addr::unchecked("sender"); // setting sender address

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    }); 
    // setting up the app with the bank module
    // You would have to change how you create an app to use the bank module (allows to set initial balances for your accounts)
    // init_balance is a function that initializes the balance of the sender, here we are setting the sender's balance to 10 atom

    let contract_id = app.store_code(counting_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &InstantiateMsg {
                counter: 0,
                minimal_donation: coin(10, "atom"),
            },
            &[],
            "Counting contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("sender"),
        contract_addr.clone(),
        &ExecMsg::Donate {},
        &coins(10, "atom"),
    )
    .unwrap();

    let resp: ValueResp = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Value {})
        .unwrap();

    assert_eq!(resp, ValueResp { value: 1 });
}

#[test]
fn expecting_no_funds() { // test to check if the contract is able to receive funds, expect no funds
    let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(0, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 }); // donate function is executed and counter is incremented if no funds are sent (contract.rs)
}

#[test]
fn withdraw() {
    let owner = Addr:: unchecked("owner");
    let sender = Addr::unchecked("sender");

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(10, "atom"))
            .unwrap();
    });

    let contract_id = app.store_code(counting_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            owner.clone(),
            &InstantiateMsg {
                counter: 0,
                minimal_donation: coin(10, "atom"),
            },
            &[],
            "Counting contract",
            None,
        )
        .unwrap();
        // instantiating the contract
 
        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        )
        .unwrap(); 
        // executing the donate function

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::Withdraw {},
            &[],
        )
        .unwrap();
    // executing the withdraw function
    // withdraw function is executed and funds are sent to the owner if the owner is the sender

 assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(10, "atom")
        );// asserting the owner has 10 atom after the withdraw function is executed

    assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
    // asserting the sender has no funds after the withdraw function is executed
    // uses ! to compare the sender's balance to an empty vector

    assert_eq!(
        app.wrap().query_all_balances(contract_addr).unwrap(),
        vec![] 
    ) // vec![] is an empty vector that is returned if the contract has no funds
    // asserting the contract address has no funds after the withdraw function is executed
    }
}
// look over