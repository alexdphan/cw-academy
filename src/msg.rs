// use cosmwasm_std::Coin;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};
// QueryResponses is a type that represents a list of query responses
use cosmwasm_schema::QueryResponses; 

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[cw_serde]
pub struct Parent {
    pub addr: String,
    pub donating_period: u64,
    pub part: Decimal,
}// added parent struct (inlcluded in InstantiateMsg, which is an Option type, meaning it can be None or Some)

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
#[cw_serde] 
pub struct InstantiateMsg { // struct to hold the data for the contract initialization (from state.rs)
    #[serde(default)]
    pub counter: u64,
    pub minimal_donation: Coin,
    pub parent: Option<Parent>,
}// added parent field which is an Option type, meaning it can be None or Some
// and it is a Parent struct, which is a struct that holds the address of the parent, the donating period and the part of the donation that the parent will receive
// added embedded struct Parent to the InstantiateMsg struct in order to keep ingo about forwarding (donations) to the parent contract. If this is None, then the contract will not forward any donations to the parent contract.

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
#[cw_serde] 
// using this instead of the above, generates all the boilerplate code for us
#[derive(QueryResponses)]
// QueryResponses is a type that represents a list of query responses
pub enum QueryMsg {
    #[returns(ValueResp)]
    // #[returns(u64)] // returns a u64 value, #[returns()] comes from cosmwasm_schema. #[derive(QueryResponses)]
    // The #[returns(...)] attribute is now required on every query variant - it describes what response type is returned for the particular query.
    Value {},
}

// Execution message to update the internal contract counter
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
#[cw_serde] // using this instead of the above, generates all the boilerplate code for us
pub enum ExecMsg {
  Donate {},
  Reset {
    #[serde(default)] 
    counter: u64,
  }, // Reset taking a counter as an argument
  Withdraw {},
  WithdrawTo {
    receiver: String, 
    #[serde(default)] // default value is an empty vector
    funds: Vec<Coin>, // Vec<Coin> is a vector of coins
  },
} 

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
#[cw_serde] // using this instead of the above, generates all the boilerplate code for us
pub struct ValueResp {
    pub value: u64,
}

#[cw_serde]
pub struct MigrateMsg {
    pub parent: Option<Parent>,
}
// added parent field which is an Option type, meaning it can be None or Some
// this message is used to migrate the contract to a new version

// --------- ADDITIONAL NOTES ------------ // 
// Msg is a type that represents a list of execution messages
// we define an enum with a single variant per execution message we want to handle. 
// #[serde(default)] is a macro that allows us to set a default value for a field in a struct or enum variant.
// The serde crate is a library that provides a way to serialize and deserialize Rust data structures in a variety of formats, such as JSON, YAML, and XML.
// When the contract is queried, it should be able to create a variety of queries. To do so, we typically create query messages as enum types, where every single variant is a separate query the contract understands.
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)] // Removed Eq as it is not needed
// #[serde(rename_all = "snake_case")]
 // #[cw_serde] using this instead of the above, generates all the boilerplate code for us instead of having to write #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)] for every struct