use cosmwasm_schema::write_api;
use counting_contract::msg::{ExecMsg, InstantiateMsg, QueryMsg};

// generating schema using the write_api! macro and the types we defined in msg.rs
fn main() {
  write_api! {
    // write_api! macro takes a list of types and generates a schema file for each of them.
    instantiate: InstantiateMsg, 
    execute: ExecMsg,
    query: QueryMsg,
  }
  // takes InstantiateMsg, ExecMsg, and QueryMsg as an argument for message type 
  // generates a schema file for it
}

// ---------- ADDITIONAL NOTES ---------- //
// By putting our file in bin subdirectory, cargo recognizes it as an entry point for a binary. 
// The only thing we need to do there is to execute a write_api macro with information about what message type is used for each entry point. 
// Based on that, it would generate a schema file containing metainformation about the contract, JSON schemas for all messages, and the relationship between queries and their responses.

// Can do cargo schema since we added the following to Cargo.toml:
// cargo run schema                                                                                                                                        ─╯
//    Compiling counting_contract v0.1.0 (/Users/alexanderphan_1/Developer/cw-academy/counting_contract)
//     Finished dev [unoptimized + debuginfo] target(s) in 1.46s
//      Running `target/debug/schema schema`
// Exported the full API as /Users/alexanderphan_1/Developer/cw-academy/counting_contract/schema/counting_contract.json

// Optional to add schema directory to .gitignore file

// In cargo build we used cargo wasm to build all the targets, including binaries (compiled executable program that can be run on a computer)

  // Binaries: Compiled executable program that can be run on a computer 
    // Example: main.rs that runs functions on its own, not as a library
    // write_api would fail to compile on its own as a schema file for a binary target

  // Libraries: A collection of code that can be used by other programs
    // Example: a static lib that contains a number of crates and can be linked to other programs

// Added --lib argument for wasm targets - so only library target is built