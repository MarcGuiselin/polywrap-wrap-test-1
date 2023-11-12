use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    JSONString,
    BigIntWrapper,
    ByteBuf
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PongResult {
    pub response: String,
}

impl PongResult {
    pub fn new() -> PongResult {
        PongResult {
            response: String::new(),
        }
    }
}
