// here, we keep all the contract proxy helpers for our test
// A contract proxy is a smart contract that acts as an intermediary between a user and a target contract. 
// The proxy contract is deployed to the blockchain and can be interacted with by users. 

use cosmwasm_std::{Addr, Coin, StdResult};
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::error::ContractError;
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};
use crate::{execute, instantiate, query};

pub struct CountingContract(Addr);
// Creating the proxy type

impl CountingContract {
  pub fn addr(&self) -> &Addr {
    &self.0
  }
  // Adding utilities to get access to the underlying address
 pub fn store_code(app: &mut App) -> u64 {
    let contract = ContractWrapper::new(execute, instantiate, query);
    app.store_code(Box::new(contract))
  }
  // Store the code of the contract in the blockchain, loading its contract code

  #[track_caller]
// track_caller: if the test fails, it will show the line number of the test that failed
  pub fn instantiate(
    app:&mut App,
    code_id: u64,
    sender: &Addr, // made it as a borrow instead of owned value (clone), saves memory
    label: &str,
    counter: impl Into<Option<u64>>,
    minimal_donation: Coin,
  ) -> StdResult<Self> {
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
      None,
    )
    .map(CountingContract)
    .map_err(|err| err.downcast().unwrap())
  }
  // Instantiate the contract, passing the code_id, sender, label, counter, and minimal_donation
  // We can eliminate arguments we don't need for our contract, in this case, we don't need funds, so we pass an empty slice, and we don't need an admin, so we pass None
  // .map_err(|err| err.downcast().unwrap()) convert the error type to the one we want, in this case, we want to convert the error type and return the error exactly as it is

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