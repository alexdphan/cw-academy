// using our new utilities in tests

// a new module to keep all the tests there and have a more ordered codebase

use cosmwasm_std::{coin, coins, Addr};
use cw_multi_test::App; 

use crate::error::ContractError;

use crate::msg::ValueResp;

use super::contract::CountingContract;

const ATOM: &str = "atom";

#[test]
fn query_value() {
  let owner = Addr::unchecked("owner");
  let mut app
}

#[test]
fn donate_with_funds() {
  let owner = Addr::unchecked("owner");
  let sender = Addr::unchecked("sender");

  let mut app: App = App::new(|router, _api, storage| {
    router
        .bank
        .init_balance(storage, &sender, coins(10, ATOM))
        .unwrap();
  });
  // setting up the bank module to have some funds
  // initial balance of 10 atom for the sender

  let code_id = CountingContract::store_code(&mut app);
  let contract = CountingContract::instantiate(
    &mut app,
    code_id,
    &owner,
    "Counting Contract",
    None,
    coin(10, "atom"),
  )
.unwrap();
// storing the code and instantiating the contract
// the contract will have 10 atom, the owner will be the owner

contract
    .donate(&mut app, &sender, &coins (10, ATOM))
    // this would increment the counter by 1
    .unwrap();
    // the sender will donate 10 atom to the contract

    let resp = contract.query_value(&app).unwrap();
    // querying the value of the contract

    assert_eq!(resp, ValueResp { value: 1 });
    // value of the contract should be 1 (src/contract.rs)
    // the contract should have 1 donation (from the CountingContract struct)
    // the contract now has 20 atom (from the CountingContract struct)
    // the owner still has 10 atom (from the CountingContract struct)
    // the sender still has 0 atom (from the CountingContract struct)
}