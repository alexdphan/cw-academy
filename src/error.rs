// Creating a new custom error type

use cosmwasm_std::StdError;
use thiserror::Error;
// We can define our own error type as simple enum types
// Can be used to return errors from the contract.
// Generates a lot of boilerplate code for us, so we can use the thiserror crate to do it for us.

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {

  #[error("{0}")]
  Std(#[from] StdError),
  // StdError reps the error type from the cosmwasm_std library
  // used in the enum type

  #[error("Unauthorized - only {owner} can call it")]
  Unauthorized { owner: String },
  // Unauthorized varient in the enum type

}
// This way, we can use the ContractError type in our smart contract, still being able to return errors occurring in cosmwasm-std. 
// The additional #[from] attribute tells thiserror to generate the From trait, converting the underlying type to the error variant (in this case: impl From<StdError> for ContractError). 
// This enables using the ? operator forwarding StdError in functions returning ContractError.