// submodules, clear distinction between query and execute msgs
// can separate, but not required to be separate modules

// Modules (mod) also allow us to declare items that are only available within a given scope, rather than making them available to the entire crate.

use cosmwasm_std::{Addr, Coin, DepsMut, MessageInfo, Response, StdResult};
use cw2::{get_contract_version, set_contract_version};
use cw_storage_plus::Item;
use serde::{Serialize, Deserialize};

use crate::error::ContractError;
use crate::msg::Parent;
use crate::state::{State, STATE, PARENT_DONATION, ParentDonation};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION"); 
// notice the use of env! macro, which allows us to access environment variables at compile time, the use of const is important here to prevent mutable access (changes)

pub fn instantiate(deps: DepsMut, info: MessageInfo, counter: u64, minimal_donation: Coin, parent: Option<Parent>) -> StdResult<Response> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

  STATE.save(deps.storage, &State {
    counter,
    minimal_donation,
    owner: info.sender,
    donating_parent: parent.as_ref().map(|p| p.donating_period), 
        }, // added donating_parent field which is a countdown till the donation period ends
        // if parent is Some, we map it to the donating_period field, if not, we map it to None (coming from Option<Parent>)
    )?;

    // if Some
    if let Some(parent) = parent {
        PARENT_DONATION.save(deps.storage, 
          &ParentDonation {
            address: deps.api.addr_validate(&parent.addr)?,
            donating_parent_period: parent.donating_period,
            part: parent.part,
        })?;
    } // if parent is Some, we save it to the storage using the PARENT_DONATION key and the referred ParentDonation struct
    // we validate the address using the addr_validate function from the api module, which returns a StdResult<Addr> type, then we also save the donating_parent_ period and part fields from the Parent struct

    Ok(Response::new())
}
// instantiate contract, set contract version, save state



pub fn migrate(mut deps: DepsMut, parent: Option<Parent>) -> Result<Response, ContractError> {
    let contract_version = get_contract_version(deps.storage)?;

    if contract_version.contract != CONTRACT_NAME {
        return Err(ContractError::InvalidContract {
            contract: contract_version.contract,
        });
    }

 let resp = match contract_version.version.as_str() {
        "0.1.0" => migrate_0_1_0(deps.branch(), parent).map_err(ContractError::from)?,
        // branch function we call on deps, utility that allows having another copy of a mutable state in a single contract, like a clone() function
        "0.2.0" => migrate_0_2_0(deps.branch(), parent).map_err(ContractError::from)?,
        CONTRACT_VERSION => return Ok(Response::default()),
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

  pub fn migrate_0_1_0(deps: DepsMut, parent: Option<Parent>) -> StdResult<Response> {
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
            donating_parent: parent.as_ref().map(|p| p.donating_period),
            // parent has a function as ref, which maps the parent to the donating_period field, if parent is None, it maps it to None
            // as_ref() is a function that returns an Option<&T> type, which is a reference to the value inside the Option, this comes from the std::option::Option module
        },
    )?;

    if let Some(parent) = parent {
        PARENT_DONATION.save(
            deps.storage,
            &ParentDonation {
                address: deps.api.addr_validate(&parent.addr)?,
                donating_parent_period: parent.donating_period,
                part: parent.part,
            },
        )?;
    }
    // if parent is Some, we save it to the storage using the PARENT_DONATION key and the referred ParentDonation struct
    // we save the donating_parent_period and part fields from the Parent struct
    Ok(Response::new())
} // migrate from 0.1.0 to 0.2.0
      
// similar to instantiation, but we are loading the data from the old state and saving it to the new state

pub fn migrate_0_2_0(deps: DepsMut, parent: Option<Parent>) -> StdResult<Response> {
    #[derive(Serialize, Deserialize)]
    struct OldState {
        counter: u64,
        minimal_donation: Coin,
        owner: Addr,
    }

const OLD_STATE: Item<OldState> = Item::new("state");
// assigns the Item struct to the constant OLD_STATE, which is a state accessor from src/state.rs

 let OldState {
        counter,
        minimal_donation,
        owner,
    } = OLD_STATE.load(deps.storage)?;
    // let is used instead of const because it is used to bind a value to a variable, and we are binding the value of the load function to the variables
    // whereas const is used to bind a value to a constant, and we are binding the value of the Item struct to the constant
    // assigns OldState to the variables and sets them to the values of the load function, which uses deps.storage as an argument
    // we use OldState to load the old state from the storage, then we save it to the new state
    // we have to use the same names as the fields in the State struct, so we can use the shorthand syntax, which is the same as writing: 
    // counter: counter, minimal_donation: minimal_donation, owner: owner
STATE.save(
        deps.storage,
        &State {
            counter,
            minimal_donation,
            owner,
            donating_parent: parent.as_ref().map(|p| p.donating_period),
        },
    )?;
    // saving the old state to the new state by using the save function from STATE, which is a state accessor from src/state.rs 
    // takes deps.storage as an argument from the migrate function (DepsMut, and the new State struct, which is a struct from src/state.rs

      if let Some(parent) = parent {
        PARENT_DONATION.save(
          deps.storage,
          &ParentDonation {
            address: deps.api.addr_validate(&parent.addr)?,
            donating_parent_period: parent.donating_period,
            part: parent.part,
          },
        )?;
      } // if parent is Some, we save it to the storage using the PARENT_DONATION key and the referred ParentDonation struct

    Ok(Response::new())
    // returns a new Response struct, which is a struct from cosmwasm_std
} // migrate from 0.2.0 to 0.3.0

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
    use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, WasmMsg, to_binary};

    use crate::error::ContractError;
    use crate::msg::ExecMsg;
    use crate::state::{STATE, PARENT_DONATION};

     pub fn donate(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
      // we use DepsMut to access contract/bc storage, and we use it to write to the storage
      // we use Env to access the blockchain context, and we use it to get the current block height
      // we use MessageInfo to access the message sender, and we use it to get the sender's address

      let mut state = STATE.load(deps.storage)?;
      let mut resp = Response::new();
      // setting the state to the value of the load function, which takes deps.storage as an argument
      // setting the response to a new Response struct, which is a struct from cosmwasm_std that is used to build a response

      // |coin| is a closure, a function that can be passed as an argument to another function
      if state.minimal_donation.amount.is_zero() || info.funds.iter().any(|coin| {
        coin.denom == state.minimal_donation.denom && coin.amount >= state.minimal_donation.amount
      }) {
        state.counter += 1;
      // if the minimal donation amount is zero, or if the funds in the message info are greater than or equal to the minimal donation amount, then we increment the counter by 1

        if let Some(parent) = &mut state.donating_parent {
          *parent -= 1;
      // if the donating parent is not empty, then we decrement the parent by 1

       if *parent == 0 {
      let parent_donation = PARENT_DONATION.load(deps.storage)?;
      *parent = parent_donation.donating_parent_period;
      // if the parent is equal to 0, then we set the parent to the donating parent period, which is a field in the parent donation struct

      // *parent is a dereference operator that allows you to access the value stored in the memory location pointed to by a reference.
      // this is different from &parent, which is a reference operator that allows you to access the memory location of a value. 
      // * is pointed to by a reference, & is a reference to a value.
        // in this example, we are pointing to parent, which is a reference to a value which is a u64, and we are setting it to the donating parent period, which is a field in the parent donation struct

  let funds: Vec<_> = deps
    .querier
    .query_all_balances(env.contract.address)?
    .into_iter()
    .map(|mut coin| {
      coin.amount = coin.amount * parent_donation.part;
      coin
    })
    .collect();
  // we are setting the funds to a vector of coins, which is a struct from cosmwasm_std that is used to represent a coin
  // we then use the query_all_balances function from the querier, which is a struct from cosmwasm_std that is used to query the blockchain
  // we use the contract address from the env struct, which is a struct from cosmwasm_std that is used to access the blockchain context
  // we then use the into_iter function to iterate over the balances, which is a vector of coins
  // we then use the map function to map over the balances, which is a vector of coins, we map using a closure, which is a function that can be passed as an argument to another function
  // we then use the amount field from the coin struct, which is a struct from cosmwasm_std that is used to represent a coin
  // we then use the part field from the parent donation struct, which is a field in the parent donation struct
  // we multiply the amount by the part, which is a field in the parent donation struct and we set it to the amount field of the coin struct
  // we then use the collect function to collect the results of the map function, which is a vector of coins

    let msg = WasmMsg::Execute {
      contract_addr: parent_donation.address.to_string(),
      msg: to_binary(&ExecMsg::Donate {})?,
      funds,
    }; 
    // here we just set the message to a WasmMsg::Execute struct, which is a struct from cosmwasm_std that is used to build a wasm message
    // we set the contract address to the address field from the parent donation struct, which is a field in the parent donation struct
    // we set the message to the donate message, which is a field in the exec message struct, which is a struct from the crate::msg module, this would be turned into a binary using the to_binary function from cosmwasm_std
    // we set the funds to the funds vector, which is a vector of coins
    // the purpose of msg is to send a message to another contract, which is the parent donation address, which is a field in the parent donation struct

    resp = resp
        .add_message(msg)
        .add_attribute("donated_to_parent", parent_donation.address.to_string());
    // we add the message to the response, which is a struct from cosmwasm_std that is used to build a response
    // we also add an attribute to the response, which is a struct from cosmwasm_std that is used to build a response
      // the purpose of the attribute is to add a key-value pair to the response, which is a struct from cosmwasm_std that is used to build a response
      // the key is donated_to_parent, and the value is the address field from the parent donation struct, which is a field in the parent donation struct
    // finally, we convert the response to string and return it, assigning it to the response variable
  }      
}

      STATE.save(deps.storage, &state)?;
      // we save the state to the storage, which is a field in the deps struct, which is a struct from cosmwasm_std that is used to access the blockchain context
      }

  resp = resp 
    .add_attribute("action", "donate")
    .add_attribute("sender", info.sender.to_string())
    .add_attribute("counter", state.counter.to_string());
    // adding attributes to the wasm event (only default event type that is emitted from every execution)
      Ok(resp)
      } 

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

// A parent contract in a smart contract is a contract that serves as a template or framework for other contracts. It can define certain variables and functions that can be inherited and reused in other contracts, which are known as child contracts. This allows developers to create modular, reusable contracts that can be easily modified and customized for different use cases.

// A parent contract can also define the rules and conditions under which the child contracts are allowed to interact with it, such as by calling its functions or accessing its data. This can help ensure that the child contracts operate in a predictable and secure manner, and that they conform to the requirements and standards set by the parent contract.