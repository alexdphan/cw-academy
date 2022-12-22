use cw_storage_plus::Item;

pub const COUNTER: Item<u64> = Item::new("counter"); // key is "counter" attached to the binary data
// Item is a type accessing a single object that may exist in the blockchain storage
// The string passed to Item on instantiation is part of a key to how the data would be addressed in the blockchain. 
// Item would use this value to access data, taking care of serialization and deserialization of it, so you don't need to work on raw binary data.