use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Decimal;


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct State {
  pub counter: u64,
  pub minimal_donation: Coin,
  pub owner: Addr,
  pub donating_parent: Option<u64>,
} // added donating_parent field which is a countdown till the donation period ends

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ParentDonation {
  pub address: Addr,
  pub donating_parent_period: u64,
  pub part: Decimal,
} // added donation_parent field which is a value to be reset when it reaches 0

pub const STATE: Item<State> = Item::new("state"); // key is "state" attached to the binary data. Accessing the State value on the storage
pub const PARENT_DONATION: Item<ParentDonation> = Item::new("parent_donation"); // key is "parent_donation" attached to the binary data. Accessing the ParentDonation value on the storage


// Item would use this value to access data, taking care of serialization and deserialization of it, so you don't need to work on raw binary data.

// The string passed to Item on instantiation is part of a key to how the data would be addressed in the blockchain. 

// --------- ADDITIONAL NOTES --------- // 
// 1. Add const in state.rs
// 2. Initialize in contract.rs
// 3. Set the struct/enum type in msg.rs
// 4. Set up public entry points; execute function in lib.rs
// 5. Use the entry points and refer to execution details in contract.rs (private functions)
// 6. Test in lib.rs