#[cfg(not(feature = "library"))] // only compile the entry_point macro if the library feature is not enabled
use cosmwasm_std::entry_point; // import the entry_point macro from cosmwasm_std

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

use error::ContractError; 
// using module file error.rs for ContractError
use msg::InstantiateMsg;
// using module file msg.rs for InstantiateMsg
// the use keyword is used to bring a module into scope, so that we can use its contents

mod contract; // private because contract contains internal logic functions, contains all msg handlers 
pub mod error; // using module file error.rs
pub mod msg; // using module file msg.rs
#[cfg(any(test, feature = "tests"))]
pub mod multitest; // compile only when running tests, only when the feature tests is enabled
mod state;
// these modules can be used by other modules in the crate, but not by code outside the crate

// In summary, the use keyword is used to import symbols from other modules into the current scope, while the pub keyword is used to make a symbol public and available for use from other modules.

// In the context of a smart contract, an entry point is a function that can be called by external users or other contracts. Entry points serve as the public interface for a contract, allowing external entities to interact with it and execute its functions.
#[cfg_attr(not(feature = "library"), entry_point)] 
// used as an entry point 
// cfg_attr attribute allows you to specify a condition under which the attribute should be applied. 
// Hence, the function will be an entry point only if the feature library is not enabled.
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, info, msg.counter, msg.minimal_donation, msg.parent)
    // calls the instantiate function for contract.rs, if the feature library is not enabled
} // entry point instantiate function for contract.rs, if the feature library is not enabled
// saves the state and owner to the blockchain, response is empty, but it is a success

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::ExecMsg,
) -> Result<Response, ContractError> {
    use contract::exec;
    use msg::ExecMsg::*;

 match msg {

        Donate {} => exec::donate(deps, env, info).map_err(ContractError::Std),

        Reset { counter } => exec::reset(deps, info, counter),

        Withdraw {} => exec::withdraw(deps, env, info),

        WithdrawTo { receiver, funds } => {
            exec::withdraw_to(deps, env, info, receiver, funds)
        }
        // added map for ContractError instead of using default StdError
        // removed if all fn always return Error being ContractError
    }
}

// Deps is read-only, DepsMut is read-write on blockchain state
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
        Value {} => to_binary(&query::value(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> Result<Response, ContractError> {
    contract::migrate(deps)
} // entry point migrate function for contract.rs, if the feature library is not enabled
// returns a StdResult<Response> type, which is an alias for Result<Response, StdError>
// using contract.rs migrate function with deps as parameter

// returns binary data (JSON serialized responses instead of Response (for Query Messages)
// pub fn can be called from anywhere
// fn can only be called from within the current module

// ----------- ADDITIONAL NOTES ----------- //
// Ok(Response::new()) - creates a new response instance that will have default values for all fields
// Ok(Response::default()) - also creates a new response instance that will have default values for all fields, but it allows us to use Default::default() synctax to create a default value for any type that implements the Default trait

// In general, you can use either Response::new() or Response::default() to create a new Response instance with default values for all fields. 
// The main difference is that Response::default() uses the Default trait, which allows you to use the Default::default() syntax and may be more convenient in some cases.

// // -------------- TESTS -------------- // 
// // tests can be in another file, but we can also put them here in lib crate
// // cfg is a conditional compilation attribute, meaning that the code it wraps would be compiled-in if and only if the predicate passed to it is true. In this case, we have test predicate, which is true on cargo test runs
// #[cfg(test)]
// mod test {
//     use cosmwasm_std::{Empty, Addr, coin, coins, Storage}; // msg type for contract
//     use cw_multi_test::{App, Contract, ContractWrapper, Executor, Router}; 
//     // App: simulate the execution of a smart contract in a controlled environment
//     // Contract: simulates a contract, testing
//     // ContractWrapper: wrapper for contract (implements 'Contract' trait), used for testing

//     use crate::error::ContractError;
//     use crate::msg::{InstantiateMsg, QueryMsg, ValueResp, ExecMsg};
//     use crate::{execute, instantiate, query}; // brings the execute, instantiate, and query functions from the current crate into the current scope

//     // https://dhghomon.github.io/easy_rust/Chapter_54.html
//    fn counting_contract() -> Box<dyn Contract<Empty>> {
//         let contract = ContractWrapper::new(execute, instantiate, query);
//         Box::new(contract)
//     }
//     // Ihe counting_contract function returns a Box containing the ContractWrapper instance as a trait object. 
//     // In this case, the counting_contract function creates an instance of a struct called ContractWrapper and passes it three functions as arguments: execute, instantiate, and query. 
//     // These functions are likely implementations of the Contract trait's methods, and they will be used to execute the contract's behavior when the trait object is called.

//     #[test]
//     fn query_value() { // test to see if the query_value function works
//         let mut app = App::default(); // App instance is the soul of the testing framework, it is the blockchain simulator, and it would be an interface to all contracts on it.
        
//         let contract_id = app.store_code(counting_contract());
//         // simulates the deployment of a contract, gives back a contract id
//         // We use it right after it is created to call store_code on it. 
//         // This method takes a contract as an argument and returns a contract id (unique identifier for the contract)
//         // key-value store, contract id is the key, contract is the value
//         // Storing code is this operation of uploading smart contract binary to be stored in a blockchain state.

//         let contract_addr = app // app is a testing framework
//         // simulates the instantiation of a contract, gives back a contract address
//         .instantiate_contract( 
//                 contract_id, // contract id the contract was deployed with
//                 Addr::unchecked("sender"), // create an arbitrary address, used for testing
//                 &InstantiateMsg { counter: 10, minimal_donation: coin(10, "atom") }, // init msg, msg you send to contract when you instantiate it
//                 // but what is important - is it would not be just passed to the entry point. It would be first serialized to JSON and then deserialized back to send it to the contract.
//                 &[], // funds, usually has tokens sent with the contract instantiation
//                 "Counting Contract", // label, name of the contract
//                 None, // admins are the only addresses that can later perform migrations of smart contracts.
//             ).unwrap(); // fail if error
//         // simulates the instantiation of a contract, gives back a contract address
        
//         let resp: ValueResp = app // app is a testing framework
//         .wrap() // converts app, wrap the contract to a temporary QueryWrapper object, allowing to query the query the blockchain
//         .query_wasm_smart(contract_addr, &QueryMsg::Value {}) // query the contract with the contract address and the query message, depending who calls, they can't mod the state
//         .unwrap(); // fail if error

//         assert_eq!(resp, ValueResp { value: 10 });
//     }

//     #[test]
//     fn reset() { // test to see if the contract (counter) can be reset
//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 Addr::unchecked("sender"),
//                 &InstantiateMsg { counter: 0, minimal_donation: coin(10, "atom") },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             Addr::unchecked("sender"),
//             contract_addr.clone(),
//             &ExecMsg::Reset { counter: 10 },
//             &[],
//         )
//         .unwrap();

//         let resp: ValueResp = app
//             .wrap()
//             .query_wasm_smart(contract_addr, &QueryMsg::Value {})
//             .unwrap();

//         assert_eq!(resp, ValueResp { value: 10 });
//     }

//     #[test]
//     fn donate() { // test to see if the contract can be donated to
// let mut app = App::default(); 

// let contract_id = app.store_code(counting_contract());

// let contract_addr = app
//     .instantiate_contract(
//         contract_id,
//         Addr::unchecked("sender"),
//         &InstantiateMsg { 
//             counter: 0, 
//             minimal_donation: coin(10, "atom") },
//         &[],
//         "Counting contract",
//         None,
//     ).unwrap();

//     app.execute_contract(
//         Addr::unchecked("sender"),
//         contract_addr.clone(),
//         &ExecMsg::Donate {},
//         &[],
//     ).unwrap();
    
//     let resp: ValueResp = app
//         .wrap()
//         .query_wasm_smart(contract_addr, &QueryMsg::Value {})
//         .unwrap();

//         assert_eq!(resp, ValueResp { value: 0 });

//     }

//     #[test]
// // fn donate_with_funds() { // test to check if the contract is able to receive funds

// //     let sender = Addr::unchecked("sender"); // setting sender address

// //     let mut app = App::new(|router, _api, storage| {
// //         router
// //             .bank
// //             .init_balance(storage, &sender, coins(10, "atom"))
// //             .unwrap();
// //     }); 
// //     // setting up the app with the bank module
// //     // You would have to change how you create an app to use the bank module (allows to set initial balances for your accounts)
// //     // init_balance is a function that initializes the balance of the sender, here we are setting the sender's balance to 10 atom

// //     let contract_id = app.store_code(counting_contract());

// //     let contract_addr = app
// //         .instantiate_contract(
// //             contract_id,
// //             Addr::unchecked("sender"),
// //             &InstantiateMsg {
// //                 counter: 0,
// //                 minimal_donation: coin(10, "atom"),
// //             },
// //             &[],
// //             "Counting contract",
// //             None,
// //         )
// //         .unwrap();

// //     app.execute_contract(
// //         Addr::unchecked("sender"),
// //         contract_addr.clone(),
// //         &ExecMsg::Donate {},
// //         &coins(10, "atom"),
// //     )
// //     .unwrap();

// //     let resp: ValueResp = app
// //         .wrap()
// //         .query_wasm_smart(contract_addr, &QueryMsg::Value {})
// //         .unwrap();

// //     assert_eq!(resp, ValueResp { value: 1 });
// // }

// #[test]
// fn expecting_no_funds() { // test to check if the contract is able to receive funds, expect no funds
//     let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 Addr::unchecked("sender"),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(0, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//         app.execute_contract(
//             Addr::unchecked("sender"),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &[],
//         )
//         .unwrap();

//         let resp: ValueResp = app
//             .wrap()
//             .query_wasm_smart(contract_addr, &QueryMsg::Value {})
//             .unwrap();

//         assert_eq!(resp, ValueResp { value: 1 }); // donate function is executed and counter is incremented if no funds are sent (contract.rs)
// }

// #[test]
// fn withdraw() {
//     let owner = Addr:: unchecked("owner");
//     let sender = Addr::unchecked("sender");

//     // representing the router, api, and the storage for the App
//     let mut app = App::new(|router, _api, storage| {
//         router
//             .bank
//             .init_balance(storage, &sender, coins(10, "atom"))
//             .unwrap();
//     });

//     let contract_id = app.store_code(counting_contract());

//     let contract_addr = app
//         .instantiate_contract(
//             contract_id,
//             owner.clone(),
//             &InstantiateMsg {
//                 counter: 0,
//                 minimal_donation: coin(10, "atom"),
//             },
//             &[],
//             "Counting contract",
//             None,
//         )
//         .unwrap();
//         // instantiating the contract
 
//         app.execute_contract(
//             sender.clone(),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &coins(10, "atom"),
//         )
//         .unwrap(); 
//         // executing the donate function as the sender

//         app.execute_contract(
//             owner.clone(),
//             contract_addr.clone(),
//             &ExecMsg::Withdraw {},
//             &[],
//         )
//         .unwrap();
//     // executing the withdraw function as the owner
//     // withdraw function is executed and funds are sent to the owner if the owner is the sender

//  assert_eq!(
//             app.wrap().query_all_balances(owner).unwrap(),
//             coins(10, "atom")
//         );// asserting the owner has 10 atom after the withdraw function is executed

//     assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
//     // asserting the sender has no funds after the withdraw function is executed
//     // uses ! to compare the sender's balance to an empty vector

//     assert_eq!(
//         app.wrap().query_all_balances(contract_addr).unwrap(),
//         vec![] 
//     ) // vec![] is an empty vector that is returned if the contract has no funds
//     // asserting the contract address has no funds after the withdraw function is executed
//     }

//     #[test]
//     fn withdraw_to() {
//         let owner = Addr::unchecked("owner");
//         let sender = Addr::unchecked("sender"); 
//         let receiver = Addr::unchecked("receiver");
//         // receiver is the address that the funds will be sent to
//         // the owner is the sender of the withdraw_to function in this test

//         let mut app = App::new(|router, _api, storage| {
//             router
//                 .bank
//                 .init_balance(storage, &sender, coins(10, "atom"))
//                 .unwrap();
//         });
//         // setting up the app with the bank module, setting the sender's balance to 10 atom

//         let contract_id = app.store_code(counting_contract());
//         // storing the contract code as a contract_id using counting_contract() function

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 }, 
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();
//             // instantiating the contract with the owner as the sender, counter set to 0, and minimal_donation set to 10 atom
//             // minimal_donation is the minimum amount of funds that can be sent to the contract
//             // InstantiateMsg is the message that is sent to the contract when it is instantiated, the minimal_donation is set to 10 atom
//             // Instantiated means the contract is created and ready to be used

//         app.execute_contract(
//             sender.clone(),
//             contract_addr.clone(),
//             &ExecMsg::Donate {},
//             &coins(10, "atom"),
//         )
//         .unwrap();
//         // executing the donate function as the sender

//         app.execute_contract(
//             owner.clone(),
//             contract_addr.clone(),
//             &ExecMsg::WithdrawTo {
//                 receiver: receiver.to_string(),
//                 funds: coins(5, "atom"),
//             },
//             &[],
//         )
//         .unwrap();
//         // executing the withdraw_to function as the owner

//         assert_eq!(app.wrap().query_all_balances(owner).unwrap(), vec![]);
//         // asserting the owner has no funds after the withdraw_to function is executed
//         assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
//         // asserting the sender has no funds after the withdraw_to function is executed
//         assert_eq!(
//             app.wrap().query_all_balances(receiver).unwrap(),
//             coins(5, "atom")
//         );
//         // asserting the receiver has 5 atom after the withdraw_to function is executed
//         assert_eq!(
//             app.wrap().query_all_balances(contract_addr).unwrap(),
//             coins(5, "atom")
//         );
//         // asserting the contract address has 5 atom after the withdraw_to function is executed
//     }

//     #[test]
//     fn unauthorized_withdraw() {
//         let owner = Addr::unchecked("owner");
//         let member = Addr::unchecked("member");
//         // member is the address that is not the owner and is trying to execute the withdraw function

//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();
//             // instantiating the contract with the owner as the sender, counter set to 0, and minimal_donation set to 10 atom

//             let err = app
//                 .execute_contract(
//                     member.clone(),
//                     contract_addr.clone(),
//                     &ExecMsg::Withdraw {},
//                     &[],
//                 ).unwrap_err();
//                 // executing the withdraw function as the member

//             assert_eq!(
//                 ContractError::Unauthorized {
//                     owner: owner.into()
//                 },
//                 err.downcast().unwrap()
//                 // downcasting is converting the error to a ContractError
//             );
//             // asserting the error is ContractError::Unauthorized
//     }
//     // just want to call Withdraw by an unauthorized address user
//     // we expect it to fail using unwrap_err instead of unwrap
//     // The assert_eq! macro is used to assert that the error value is equal to a ContractError::Unauthorized error. 

//     #[test]
//     fn unauthorized_withdraw_to() {
//         let owner = Addr::unchecked("owner");

//         let member = Addr::unchecked("member");

//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//             let err = app
//                 .execute_contract(
//                     member,
//                     contract_addr,
//                     &ExecMsg::WithdrawTo {
//                         receiver: owner.to_string(),
//                        funds: vec![],
//                 },
//                 &[],
//             )
//             .unwrap_err();
//             // executing the withdraw_to function as the member towards the owner

//             assert_eq!(
//                 ContractError::Unauthorized {
//                     owner: owner.into()
//                 },
//                 err.downcast().unwrap()
//             );
//     }

//     #[test]
//     fn unauthorized_reset() {
//         let owner = Addr::unchecked("owner");
//         let member = Addr::unchecked("member");
//         // member is the address that is not the owner and is trying to execute the reset function

//         let mut app = App::default();

//         let contract_id = app.store_code(counting_contract());

//         let contract_addr = app
//             .instantiate_contract(
//                 contract_id,
//                 owner.clone(),
//                 &InstantiateMsg {
//                     counter: 0,
//                     minimal_donation: coin(10, "atom"),
//                 },
//                 &[],
//                 "Counting contract",
//                 None,
//             )
//             .unwrap();

//             let err = app
//                 .execute_contract(
//                     member,
//                     contract_addr,
//                     &ExecMsg::Reset {
//                         counter: 10,
//                     },
//                     &[],
//                 )
//                 .unwrap_err();
//                 // executing the reset function as the member

//             assert_eq!(
//                 ContractError::Unauthorized {
//                     owner: owner.into()
//                 },
//                 err.downcast().unwrap()
//             );
//     }
// }
