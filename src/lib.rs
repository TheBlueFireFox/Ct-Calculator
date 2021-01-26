pub mod api;
pub mod utils;

mod addition;
mod logical;

use addition::ADD;
pub use api::{format, Results};
use logical::{AND, OR, XOR};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

trait Supported {
    fn new4(left: i32, right: i32) -> Results;
    fn new8(left: i32, right: i32) -> Results;
    fn new16(left: i32, right: i32) -> Results;
    fn new32(left: i32, right: i32) -> Results;
}

#[wasm_bindgen]
pub fn sub(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    add(left, !right + 1, of)
}

#[wasm_bindgen]
pub fn add(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    run::<ADD>(left, right, of)
}

#[wasm_bindgen]
pub fn and(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    run::<AND>(left, right, of)
}

#[wasm_bindgen]
pub fn or(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    run::<OR>(left, right, of)
}

#[wasm_bindgen]
pub fn xor(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    run::<XOR>(left, right, of)
}

fn run<T: Supported>(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    match of {
        4 => Ok(T::new4(left, right)),
        8 => Ok(T::new8(left, right)),
        16 => Ok(T::new16(left, right)),
        32 => Ok(T::new32(left, right)),
        _ => Err(JsValue::from("unsupported value")),
    }
}
