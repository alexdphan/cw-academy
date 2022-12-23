use serde::{Deserialize, Serialize};

// When the contract is queried, it should be able to create a variety of queries. To do so, we typically create query messages as enum types, where every single variant is a separate query the contract understands.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
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
  Poke {},
  Reset {
    #[serde(default)] 
    counter: u64,
  }, // Reset taking a counter as an argument
   // #[serde(default)] is a macro that allows us to set a default value for a field in a struct or enum variant.
   // The serde crate is a library that provides a way to serialize and deserialize Rust data structures in a variety of formats, such as JSON, YAML, and XML.
} // we define an enum with a single variant per execution message we want to handle.