use serde::{Deserialize, Serialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env,
    to_vec,
    from_slice,
    JSONString,
    BigIntWrapper,
    ByteBuf
};
use crate::{ModuleTrait, Module};
use crate::PongResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsPing {
    pub message: String,
}

pub fn ping_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsPing>(args) {
        Ok(args) => {
            let result = Module::ping(ArgsPing {
                message: args.message,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
