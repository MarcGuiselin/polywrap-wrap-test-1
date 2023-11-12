use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    JSONString,
    BigIntWrapper,
    ByteBuf
};
use crate::{
    ArgsPing,
};
use crate::PongResult;

pub struct Module;

pub trait ModuleTrait {
  fn ping(args: ArgsPing) -> Result<PongResult, String>;
}
