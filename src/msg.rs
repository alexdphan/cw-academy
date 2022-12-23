use cosmwasm_std::Coin;
use serde::{Deserialize, Serialize};

// When the contract is queried, it should be able to create a variety of queries. To do so, we typically create query messages as enum types, where every single variant is a separate query the contract understands.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] // Removed Eq as it is not needed
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg { // struct to hold the data for the contract initialization (from state.rs)
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Coin 
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {},
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64,
}

// Execution message to update the internal contract counter
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
  Donate {},
  Reset {
    #[serde(default)] 
    counter: u64,
  }, // Reset taking a counter as an argument
  Withdraw {},
} // we define an enum with a single variant per execution message we want to handle. 
// #[serde(default)] is a macro that allows us to set a default value for a field in a struct or enum variant.
// The serde crate is a library that provides a way to serialize and deserialize Rust data structures in a variety of formats, such as JSON, YAML, and XML.