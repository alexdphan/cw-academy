// submodules, clear distinction between query and execute msgs
// can separate, but not required to be separate modules

// Modules (mod) also allow us to declare items that are only available within a given scope, rather than making them available to the entire crate.
pub mod query {
  use cosmwasm_std::{Deps, StdResult};
  
  use crate::msg::ValueResp;
  use crate::state::COUNTER;

    pub fn value(deps: Deps) -> StdResult<ValueResp> { // Deps to access contract/bc storage
      let value = COUNTER.load(deps.storage)?; 
      // error handling, so we use ?
      // load function to load from the state, taking state accessor as an arguement
      Ok(ValueResp { value })
    }

    // erased from lesson 5
    pub fn incremented(value: u64) -> ValueResp {
      ValueResp { value: value + 1 }
    }
  }
