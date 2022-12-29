// we use a contract proxy to simplify the tests, and to make them more readable
// proxy helpers are functions that perform actions of the contract, like instantiate, execute, query, etc
// A contract proxy is a smart contract that acts as an intermediary between a user and a target contract. 
// The proxy contract is deployed to the blockchain and can be interacted with by users. 
// More info in additional notes at the end of the file

use cosmwasm_std::{Addr, Coin, Empty, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};
use crate::{execute, instantiate, migrate, query};

pub struct CountingContract(Addr);
// Creating the proxy type

impl CountingContract {
  pub fn addr(&self) -> &Addr {
    &self.0
  }
  // Adding utilities to get access to the underlying address
pub fn store_code(app: &mut App) -> u64 {
    let contract = ContractWrapper::new(execute, instantiate, query).with_migrate(migrate);
    // assigning the contract to the contract wrapper (represents a sc on a blockchain) (with_migrate), and adding the migrate function
    app.store_code(Box::new(contract)) 
    // use app to store the code of the contract in the blockchain
    // storing the contract in the blockchain, parameter is a Box that contains the new contract
}
// Store the code of the contract in the blockchain, loading its contract code
// telling the multitest where to find the migration function

  #[track_caller]
// track_caller: if the test fails, it will show the line number of the test that failed
// info on 'a: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// https://stackoverflow.com/questions/47640550/what-is-a-in-rust-language#:~:text=The%20'a%20reads%20'the%20lifetime,which%20lifetimes%20are%20one%20kind.
  pub fn instantiate<'a>(
    app:&mut App, // borrows the mutable app from the test (in multitest), we can use it to instantiate the contract
    code_id: u64,
    sender: &Addr, // made it as a borrow instead of owned value (clone), saves memory
    label: &str,
   admin: impl Into<Option<&'a Addr>>, // passing a reference to an address, so we can pass a reference, passed an additional lifetime parameter 'a
    counter: impl Into<Option<u64>>, 
    minimal_donation: Coin,
  ) -> StdResult<Self> {
    let admin = admin.into();
       // assigning the admin value to admin. into() is a method that converts the value into the usually inferred input type
    let counter = counter.into().unwrap_or_default();
    // unwrap_or_default: if counter is not provided, use default value (can be provided from #[serde(default)]

      app.instantiate_contract(
            code_id,
            sender.clone(),
            &InstantiateMsg {
                counter,
                minimal_donation,
            },
            &[],
            label,
            admin.map(Addr::to_string),
        )
        .map(CountingContract)
        .map_err(|err| err.downcast().unwrap())
    }
  // Instantiate the contract, passing the code_id, sender, label, counter, and minimal_donation
  // We can eliminate arguments we don't need for our contract, in this case, we don't need funds, so we pass an empty slice, and we don't need an admin, so we pass None
  // .map_err(|err| err.downcast().unwrap()) convert the error type to the one we want, in this case, we want to convert the error type and return the error exactly as it is

    #[track_caller]
    pub fn migrate(app: &mut App, contract: Addr, code_id: u64, sender: &Addr) -> StdResult<Self> {
        app.migrate_contract(sender.clone(), contract.clone(), &Empty {}, code_id)
            .map_err(|err| err.downcast().unwrap()) // convert the error type and return the error exactly as it is
            .map(|_| Self(contract)) // map the result to a new instance of the contract
    }
  // Migrate the contract, passing the contract, code_id, and sender
  // Here, we don't need to pass any msg arguments, so we pass an empty struct, Empty{}
  // .map(|_| Self(contract)) map the result to a new instance of the contract, passing the contract, returning contract address wrapped in the new helper type

 #[track_caller]
 // track_caller: if the test fails, it will point to where the function was called, not where the error (panic) was thrown
      pub fn donate(
        &self,
        app: &mut App,
        sender: &Addr,
        funds: &[Coin],
    ) -> Result<(), ContractError> {
        app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Donate {}, funds)
            .map_err(|err| err.downcast().unwrap()) // convert the error type and return the error exactly as it is
            .map(|_| ()) // map the result to a unit type, which is a type that has only one value, which is ()
    } // function to donate to the contract, passing the sender, and funds
// Execute the contract, passing the sender, and funds
// map a result because we don't care about the result (not useful, interesting rn), we just want to check if the execution was successful or not

#[track_caller]
pub fn reset(
  &self,
  app: &mut App,
  sender: &Addr,
  counter: impl Into<Option<u64>>,
) -> Result<(), ContractError> {
  let counter = counter.into().unwrap_or_default();
  app.execute_contract(
    sender.clone(),
    self.0.clone(),
    &ExecMsg::Reset { counter },
    &[],
  )
.map_err(|err| err.downcast().unwrap())
.map(|_| ())
}
// function to reset the counter, passing the sender, and counter

#[track_caller]
pub fn withdraw(&self, app: &mut App, sender: &Addr) -> Result<(), ContractError> {
  app.execute_contract(sender.clone(), self.0.clone(), &ExecMsg::Withdraw {}, &[])
    .map_err(|err| err.downcast().unwrap())
    .map(|_| ())
}
// function to withdraw the funds, passing the sender

#[track_caller]
pub fn withdraw_to(
  &self,
  app: &mut App, 
  // mutable reference to the app struct argument, & is used to borrow the app
  sender: &Addr, // borrow sender address
  receiver: &Addr, // borrow receiver address
  funds: impl Into<Option<Vec<Coin>>>,
) -> Result<(), ContractError> {
  let funds = funds.into().unwrap_or_default();
    app.execute_contract(
            sender.clone(),
            self.0.clone(),
            &ExecMsg::WithdrawTo {
                receiver: receiver.to_string(),
                funds,
            },
            &[],
        )
.map_err(|err| err.downcast().unwrap())
.map(|_| ())
}
// function to withdraw the funds to a specific address, passing the sender, receiver, and funds


    #[track_caller]
    pub fn query_value(&self, app: &App) -> StdResult<ValueResp> {
        app.wrap()
            .query_wasm_smart(self.0.clone(), &QueryMsg::Value {})
    }
    // we don't change blockchain state, so we don't need a mutable reference to the app (mut &App)
    // Obviously, for queries, we need to return some reasonable data - the result of the query.
    // query_value in the CountingContract struct, referencing the App struct and returning a StdResult<ValueResp> struct
    // The body of the query_value method calls the query_wasm_smart method on the app argument and passes it the Addr field of the CountingContract struct and a QueryMsg::Value message. 
    // The query_wasm_smart method is a method of the App struct that is used to query a smart contract.

} // all the methods are in the impl block

impl From<CountingContract> for Addr {
  fn from(contract: CountingContract) -> Self {
    contract.0
  }
// No Clone trait, as the contract represents a contract instantiate on the blockchain
}




// ----------------- ADDITIONAL NOTES ----------------- //
// The newtype idiom gives compile time guarantees that the right type of value is supplied to a program.

// A contract proxy is a smart contract that acts as an intermediary between a user and a target contract. 
// The proxy contract is deployed to the blockchain and can be interacted with by users. When a user sends a message to the proxy contract, the proxy contract forwards the message to the target contract and returns the response to the user.

// track_caller: if the test fails, it will point to where the function was called, not where the error (panic) was thrown
// if you have a call of instantiating in the contract, and the test fails because of panic, you will not see a panic being in the err.downcast().unwrap() line, but instead, it would be in the line where instantiate is called in the test. I use this attribute on every test helper which contains any panicking function - it vastly improves test debugability on some strange assumption breaks.

// -------------------------- // 

// these are the same functions we have in src/contract.rs, but we are using them in a different way
// the code here is more compact and readable than the code in src/contract.rs
// For example, we don't always have to instantiate a contract, we can just use the proxy to instantiate it once (contract.rs) and then use use the instantiated contract proxy in the tests (tests.rs)

// Overall, its preference to choose doing multitests or unit tests

// in this multi-test, entrypoint is in lib.rs, functions are in contract.rs, tests are in tests.rs. The modules are in the file multitest.rs

// in unit tests, entrypoint and tests are in lib.rs, functions are in contract.rs

// both use entrypoints from lib.rs and execution messages from msg.rs