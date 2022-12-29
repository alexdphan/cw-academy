// submodules, clear distinction between query and execute msgs
// can separate, but not required to be separate modules

// Modules (mod) also allow us to declare items that are only available within a given scope, rather than making them available to the entire crate.

use cosmwasm_std::{Addr, Coin, DepsMut, MessageInfo, Response, StdResult};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Item;

use crate::error::ContractError;
use crate::state::{State, STATE};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); 
// notice the use of env! macro, which allows us to access environment variables at compile time, the use of const is important here to prevent mutable access (changes)

pub fn instantiate(deps: DepsMut, info: MessageInfo, counter: u64, minimal_donation: Coin) -> StdResult<Response> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

  STATE.save(deps.storage, &State {
    counter,
    minimal_donation,
    owner: info.sender,
        },
    )?;
    Ok(Response::new())
}
// initialize contract state

pub fn migrate(mut deps: DepsMut) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;

    if contract_version.contract != CONTRACT_NAME {
        return Err(ContractError::InvalidContract {
            contract: contract_version.contract,
        });
    }

 let resp = match contract_version.version.as_str() {
        "0.1.0" => migrate_0_1_0(deps.branch()).map_err(ContractError::from)?,
        // branch function we call on deps, utility that allows having another copy of a mutable state in a single contract, like a clone() function
        "0.2.0" => return Ok(Response::default()), // no migration needed since we are already at 0.2.0
        version => {
            return Err(ContractError::InvalidContractVersion {
                version: version.into(),
            })
        } // finally, we update the contract version to the new value so the contract version would be valid on future migrations
    };
    
    // loaded version of the contract, then we validate if the contract name didn't change
    // if it did, we return an error, if not we match the version with the migrate functions
    // works if CONTRACT_VERSION is a constant, if its a variable, it would be treated as a generic match, and the last branch would be unreachable

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(resp)
  } // migrate contract state to a different version (0.1.0 or 0.2.0)
  // It would also be a great idea to keep all of this in its own migration module. Then create another migrate function, performing the version dispatch:

  pub fn migrate_0_1_0(deps: DepsMut) -> StdResult<Response> {
    const COUNTER: Item<u64> = Item::new("counter");
    const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
    const OWNER: Item<Addr> = Item::new("owner");
    let counter = COUNTER.load(deps.storage)?;
    let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;
    let owner = OWNER.load(deps.storage)?;
    STATE.save(
        deps.storage,
        &State {
            counter,
            minimal_donation,
            owner,
        },
    )?;
    Ok(Response::new())
} // migrate contract state to new version, migrate to 0.1.0
      
// similar to instantiation, but we are loading the data from the old state and saving it to the new state

// query is a read operation
pub mod query {
  use cosmwasm_std::{Deps, StdResult};
  
  use crate::msg::ValueResp;
  use crate::state::STATE;

    pub fn value(deps: Deps) -> StdResult<ValueResp> { // Deps to access contract/bc storage
      let value = STATE.load(deps.storage)?.counter; 
      // error handling, so we use ?
      // load function, loading from the state, taking state accessor as an arguement
      Ok(ValueResp { value })
    }
  } 
  // load vs save: load is a read-only operation, save is a write operation.
  // we have a function called value, which takes a Deps argument and returns a result of type StdResult<ValueResp>
  // it is a read-only operation (pub mod query), so we use the load function on the STATE constant to load the value from the contract's storage.

  // execute is a write operation
  pub mod exec {
    use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};

    use crate::error::ContractError;
    use crate::state::STATE;

     pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // don't always want to update counter, delayed updating the counter, make it mutable
      let mut state = STATE.load(deps.storage)?;

        // |coin| is a closure, a function that can be passed as an argument to another function
      if state.minimal_donation.amount.is_zero() || info.funds.iter().any(|coin| {
        coin.denom == state.minimal_donation.denom && coin.amount >= state.minimal_donation.amount
      }) {
        state.counter += 1;
        STATE.save(deps.storage, &state)?;
      }
      // function checks whether the amount field of the MINIMAL_DONATION struct is zero, or if there is a coin in the funds field of the MessageInfo struct with a denom matching the denom field of the MINIMAL_DONATION struct and an amount greater than or equal to the amount field of the MINIMAL_DONATION struct. 
      // If either of these conditions is true, the counter variable is incremented by 1 and the updated value is saved back to the contract's storage using the save method on the COUNTER constant.

          let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", state.counter.to_string());
          // adding attributes to the wasm event (only default event type that is emitted from every execution)
      Ok(resp)
      } 
    // similar to instantiate, but update/incrementing the counter
    // this function, poke, there is a storage and info (sender) being passed as an argument to the save method of the COUNTER object.
    // returns a result of type StdResult<Response>

        pub fn reset(deps: DepsMut, info: MessageInfo, counter: u64) -> Result<Response, ContractError> {
         let mut state = STATE.load(deps.storage)?;
         if info.sender != state.owner {
           return Err(ContractError::Unauthorized {
            owner: state.owner.to_string(),
           });
         } 

         state.counter = counter;
          STATE.save(deps.storage, &state)?;

        let resp = Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

                    Ok(resp)
  } // reset counter to 0
  // Withdraws unthouched

  pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let owner = STATE.load(deps.storage)?.owner;
    if info.sender != owner {
      return Err(ContractError::Unauthorized {
       owner: owner.to_string(),
      });
    } // checking if the sender of the message is the owner/creator of the contract
    // instead of returning a generic error (StdError::generic_error(...)), we return a custom error, which is a ContractError::Unauthorized.

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

  pub fn withdraw_to(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receiver: String,
    funds: Vec<Coin>,
  ) -> Result<Response, ContractError> {
    let owner = STATE.load(deps.storage)?.owner;
    if info.sender != owner {
      return Err(ContractError::Unauthorized {
        owner: owner.to_string(),
      });
    } // checking if the sender of the message is the owner/creator of the contract

    let mut balance = deps.querier.query_all_balances(&env.contract.address)?; // assign balance to the balance of the contract

    if !funds.is_empty() { // if funds is not empty
      for coin in &mut balance {
         // for each coin in balance
        let limit = funds 
        // limit is the amount of the coin in funds
        .iter() 
        // iterates through the funds
        .find(|c| c.denom == coin.denom) 
        // finds the coin with the same denom as the coin in funds
        .map(|c| c.amount) 
        // maps the amount of the coin in funds
        .unwrap_or(Uint128::zero()); 
        // if there is no coin with the same denom as the coin in funds, set the amount to zero

        coin.amount = std::cmp::min(coin.amount, limit);
        // set the amount to the minimum of the two amounts to prevent withdrawing more than the limit
      }
    } // if funds is not empty, iterate through the balance and find the coin with the same denom as the coin in funds, and set the amount to the minimum of the two amounts

    let bank_msg = BankMsg::Send {
      to_address: receiver,
      amount: balance,
    }; // uses BankMsg::Send to send the balance to the receiver, adding it to the response object

    let resp = Response::new()
        .add_message(bank_msg)
        .add_attribute("action", "withdraw")
        // withdraw in add_attribut is the action that allows the owner to withdraw funds from the contract
        .add_attribute("sender", info.sender.as_str());
        // sender is the owner of the contract, which is the sender of the message, which is the only one who can execute this function
        Ok(resp) 
    } 
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