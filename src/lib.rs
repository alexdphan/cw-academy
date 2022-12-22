use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
mod contract; // private because contract contains internal logic functions, contains all msg handlers 
pub mod msg; // using module file msg.rs

// In the context of a smart contract, an entry point is a function that can be called by external users or other contracts. Entry points serve as the public interface for a contract, allowing external entities to interact with it and execute its functions.
#[entry_point]
pub fn instantiate(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    // Ok(Response::default())
    Ok(Response::new())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

// Deps is read-only, DepsMut is read-write on blockchain state
#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    use contract::query;
    use msg::QueryMsg::*;

    match msg {
       Value {} => to_binary(&query::value()),
       Incremented { value } => to_binary(&query::incremented(value)),
    }
}
// returns binary data (JSON serialized responses instead of Response (for Query Messages)
// pub fn can be called from anywhere
// fn can only be called from within the current module