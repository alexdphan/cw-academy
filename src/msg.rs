use serde::{Deserialize, Serialize};

// When the contract is queried, it should be able to create a variety of queries. To do so, we typically create query messages as enum types, where every single variant is a separate query the contract understands.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg { 
  Value {},
  Incremented { value: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")] // for making the JSON keys snake_case
pub struct ValueResp {
  pub value: u64,
}