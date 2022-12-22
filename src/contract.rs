// submodules, clear distinction between query and execute msgs
// can separate, but not required to be separate modules
pub mod query {
  use crate::msg::ValueResp;

    pub fn value() -> ValueResp {
      ValueResp { value: 0 }
    }

    pub fn incremented(value: u64) -> ValueResp {
      ValueResp { value: value + 1 }
    }
  }
