use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

pub const COUNTER: Item<u64> = Item::new("counter"); // key is "counter" attached to the binary data
// Item is a type accessing a single object that may exist in the blockchain storage
// The string passed to Item on instantiation is part of a key to how the data would be addressed in the blockchain. 
// Item would use this value to access data, taking care of serialization and deserialization of it, so you don't need to work on raw binary data.
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
// A new Item, accessing the Coin value on the storage. 
// Coin is a type representing a single native token amount containing a denominator (its unique identifier) and number of tokens sent
pub const OWNER: Item<Addr> = Item::new("owner");
// Addr is a type representing a Cosmos SDK address, which is a 20-byte value that is used to identify an account on the Cosmos Hub.




// --------- ADDITIONAL NOTES --------- // 
// 1. Add const in state.rs
// 2. Initialize in contract.rs
// 3. Set the struct/enum type in msg.rs
// 4. Set up public entry points; execute function in lib.rs
// 5. Use the entry points and refer to execution details in contract.rs (private functions)
// 6. Test in lib.rs