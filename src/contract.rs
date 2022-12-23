// submodules, clear distinction between query and execute msgs
// can separate, but not required to be separate modules

// Modules (mod) also allow us to declare items that are only available within a given scope, rather than making them available to the entire crate.

use cosmwasm_std::{Coin, DepsMut, MessageInfo, Response, StdResult};

use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

pub fn instantiate(deps: DepsMut, info: MessageInfo, counter: u64, minimal_donation: Coin)  -> StdResult<Response> {
  COUNTER.save(deps.storage, &counter)?;
  MINIMAL_DONATION.save(deps.storage, &minimal_donation)?; // minimal_donation comes from Coin
  OWNER.save(deps.storage, &info.sender)?; // info.sender comes from MessageInfo
  Ok(Response::new())
} // initialize contract state

// query is a read operation
pub mod query {
  use cosmwasm_std::{Deps, StdResult};
  
  use crate::msg::ValueResp;
  use crate::state::COUNTER;

    pub fn value(deps: Deps) -> StdResult<ValueResp> { // Deps to access contract/bc storage
      let value = COUNTER.load(deps.storage)?; 
      // error handling, so we use ?
      // load function, loading from the state, taking state accessor as an arguement
      Ok(ValueResp { value })
    }
  } 
  // load vs save: load is a read-only operation, save is a write operation.

  // execute is a write operation
  pub mod exec {
    use cosmwasm_std::{BankMsg, Env};
    use cosmwasm_std::{DepsMut, Response, MessageInfo, StdResult, StdError};

use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

     pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // don't always want to update counter, delayed updating the counter, make it mutable
        let mut counter = COUNTER.load(deps.storage)?; 
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        // |coin| is a closure, a function that can be passed as an argument to another function
      if minimal_donation.amount.is_zero() || info.funds.iter().any(|coin| {
        coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
      }) {
        counter += 1;
        COUNTER.save(deps.storage, &counter)?;
      }
      // function checks whether the amount field of the MINIMAL_DONATION struct is zero, or if there is a coin in the funds field of the MessageInfo struct with a denom matching the denom field of the MINIMAL_DONATION struct and an amount greater than or equal to the amount field of the MINIMAL_DONATION struct. 
      // If either of these conditions is true, the counter variable is incremented by 1 and the updated value is saved back to the contract's storage using the save method on the COUNTER constant.

          let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());
          // adding attributes to the wasm event (only default event type that is emitted from every execution)
      Ok(resp)
      }
    // similar to instantiate, but update/incrementing the counter
    // this function, poke, there is a storage and info (sender) being passed as an argument to the save method of the COUNTER object.
    // returns a result of type StdResult<Response>

        pub fn reset(deps: DepsMut, info: MessageInfo, counter: u64) -> StdResult<Response> {
        COUNTER.save(deps.storage, &counter)?;

        let resp = Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

                    Ok(resp)
  } // reset counter to 0

  pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
      return Err(StdError::generic_err("Unauthorized, Only owner can withdraw"));
    } // checking if the sender of the message is the owner/creator of the contract

    let balance = deps.querier.query_all_balances(&env.contract.address)?;
    let bank_msg = BankMsg::Send {
      to_address: info.sender.to_string(),
      amount: balance,
    }; // queried all the balances of the contract, getting contract address using env and sent them to the sender of the message
    // uses BankMsg::Send to send the balance to the sender of the message to the owner, which is the sender of the message

    let resp = Response::new()
    // add_message function, which takes a Cosmos SDK message as an argument and adds it to the Response object
        .add_message(bank_msg) 
        .add_attribute("action", "withdraw")
        .add_attribute("sender", info.sender.as_str());

        Ok(resp)
  } 
  // withdraw all funds from contract while adding a message to the response, submessages must be processed first, then the Response object is returned. If submessages are not processed, the Response object is not returned.
  // uses bank_msg to send the balance to the sender of the message to the owner, which is the sender of the message, adding it to the response object
}

// --------- Additional Notes --------- //

// Events are emitted from execution using the Response::add_event function, passing the constructed Event type.

// Every execution emits at least one default event, with the type of wasm. In most cases, it is good enough to emit only that one. To add attributes to the wasm event, we can use a Response::add_attribute function. That is what we would do in our contract:

// use .update to update the state if you don't need the old or update value,
// use .save if you want to update the state also have the old value

// MessageInfo. It contains additional metadata about the sent message - the message sender and the funds sent. It is passed to the execute function as an argument.  That is the proper way to detect the actual sender of the message

// Finally, before returning the Response object, we added three attributes to it - action, sender, and counter. 
// action and sender are pretty much standard, and I encourage you to set it on every single execution your contract perform. 
// The counter is very specific to the contract.