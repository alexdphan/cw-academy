use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct State {
  pub counter: u64,
  pub minimal_donation: Coin,
  pub owner: Addr,
}

pub const STATE: Item<State> = Item::new("state"); // key is "state" attached to the binary data. Accessing the State value on the storage
pub const OWNER: Item<Addr> = Item::new("owner"); // key is "owner" attached to the binary data. Accessing the Addr value on the storage

// Item would use this value to access data, taking care of serialization and deserialization of it, so you don't need to work on raw binary data.

// The string passed to Item on instantiation is part of a key to how the data would be addressed in the blockchain. 

// --------- ADDITIONAL NOTES --------- // 
// 1. Add const in state.rs
// 2. Initialize in contract.rs
// 3. Set the struct/enum type in msg.rs
// 4. Set up public entry points; execute function in lib.rs
// 5. Use the entry points and refer to execution details in contract.rs (private functions)
// 6. Test in lib.rs