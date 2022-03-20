pub mod api;
pub mod utils;

mod addition;
mod logical;

use addition::Add;
pub use api::{format, Results};
use logical::{And, Nand, Or, Xor};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

trait Supported {
    fn new4(left: i32, right: i32) -> Results;
    fn new8(left: i32, right: i32) -> Results;
    fn new16(left: i32, right: i32) -> Results;
    fn new32(left: i32, right: i32) -> Results;
}

macro_rules! runner {
    ($name:ident, $fun:ident) => {
        #[wasm_bindgen]
        pub fn $name(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
            run::<$fun>(left, right, of)
        }
    };
}

#[wasm_bindgen]
pub fn sub(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    add(left, !right + 1, of)
}

runner!(add, Add);
runner!(and, And);
runner!(nand, Nand);
runner!(or, Or);
runner!(xor, Xor);

fn run<T: Supported>(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    match of {
        4 => Ok(T::new4(left, right)),
        8 => Ok(T::new8(left, right)),
        16 => Ok(T::new16(left, right)),
        32 => Ok(T::new32(left, right)),
        _ => Err(JsValue::from("unsupported value")),
    }
}
