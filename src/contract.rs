// submodules, clear distinction between query and execute msgs
// can separate, but not required to be separate modules

// Modules (mod) also allow us to declare items that are only available within a given scope, rather than making them available to the entire crate.

use cosmwasm_std::{DepsMut, Response, StdResult};

use crate::state::COUNTER;

pub fn instantiate(deps: DepsMut, counter: u64) -> StdResult<Response> {
  COUNTER.save(deps.storage, &counter)?;
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
    use cosmwasm_std::{DepsMut, Response, MessageInfo, StdResult};

    use crate::state::COUNTER;

     pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let counter = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &counter)?;

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