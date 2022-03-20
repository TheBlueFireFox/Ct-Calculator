use utils::to_i4;
use wasm_bindgen::prelude::*;

use crate::utils;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Results {
    flags: ResultFlags,
    values: ResultValue,
}

#[wasm_bindgen]
impl Results {
    pub fn new(flags: ResultFlags, values: ResultValue) -> Self {
        Self { flags, values }
    }

    #[wasm_bindgen(getter)]
    pub fn get_flags(&self) -> ResultFlags {
        self.flags.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_value(&self) -> ResultValue {
        self.values.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ResultFlags {
    pub zero: bool,
    pub negative: bool,
    pub overflow: bool,
    pub carry: bool,
    pub borrow: bool,
}

impl ResultFlags {
    pub fn new(zero: bool, negative: bool, overflow: bool, carry: bool) -> Self {
        Self {
            zero,
            negative,
            overflow,
            carry,
            borrow: !carry,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FormattedValue {
    signed: String,
    unsigned: String,
    bin: String,
    com: String,
    hex: String,
}

#[wasm_bindgen]
impl FormattedValue {
    #[wasm_bindgen(getter)]
    pub fn get_signed(&self) -> String {
        self.signed.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_unsigned(&self) -> String {
        self.unsigned.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_bin(&self) -> String {
        self.bin.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_com(&self) -> String {
        self.com.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_hex(&self) -> String {
        self.hex.clone()
    }
}

#[wasm_bindgen]
pub fn format(value: i32, of: i32) -> Result<FormattedValue, JsValue> {
    match of {
        4 => {
            let value = value as u8;
            let com = !value + 1;
            let svalue = to_i4(value);
            Ok(FormattedValue::new4(value, svalue, com))
        }
        8 => {
            let value = value as u8;
            let com = !value + 1;
            let svalue = value as i8;
            Ok(FormattedValue::new(value, svalue, com))
        }
        16 => {
            let value = value as u16;
            let com = !value + 1;
            let svalue = value as i16;
            Ok(FormattedValue::new(value, svalue, com))
        }
        32 => {
            let value = value as u32;
            let com = !value + 1;
            let svalue = value as i32;
            Ok(FormattedValue::new(value, svalue, com))
        }
        _ => Err(JsValue::from("unsupported value")),
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ResultValue {
    signed: String,
    unsigned: String,
    bin: String,
    hex: String,
}

mod formatter {
    use crate::api::FormattedValue;

    use super::ResultValue;
    use std::fmt::{Binary, Display, UpperHex};

    impl FormattedValue {
        pub fn new4(unsigned: u8, signed: i8, complement: u8) -> Self {
            let res = ResultValue::new4(unsigned, signed);
            let s = format!("{:b}", complement);
            let mut comp = fix_size::<u8>(s, 8);
            comp = (&comp[comp.len() - 4..]).to_string();

            Self {
                signed: res.signed,
                unsigned: res.unsigned,
                bin: res.bin,
                hex: res.hex,
                com: comp,
            }
        }

        pub fn new<U, S>(unsigned: U, signed: S, complement: U) -> Self
        where
            U: num::Unsigned + Display + UpperHex + Binary,
            S: num::Signed + Display,
        {
            let res = ResultValue::new(unsigned, signed);
            let s = format!("{:b}", complement);
            let comp = fix_size::<U>(s, 8);
            Self {
                signed: res.signed,
                unsigned: res.unsigned,
                bin: res.bin,
                hex: res.hex,
                com: comp,
            }
        }
    }

    fn fix_size<T>(s: String, mult: usize) -> String {
        let size = std::mem::size_of::<T>() * mult;
        format!("{}{}", "0".repeat(size - s.len()), s)
    }

    /// is split here as to have a pub new, while having some wasm_bindgen
    /// getters.
    impl ResultValue {
        pub fn new<U, S>(unsigned: U, signed: S) -> Self
        where
            U: num::Unsigned + Display + UpperHex + Binary,
            S: num::Signed + Display,
        {
            Self {
                unsigned: format!("{}", unsigned),
                signed: format!("{}", signed),
                hex: fix_size::<U>(format!("{:X}", unsigned), 2),
                bin: fix_size::<U>(format!("{:b}", unsigned), 8),
            }
        }

        pub fn new4(unsigned: u8, signed: i8) -> Self {
            let mut res = Self::new(unsigned, signed);
            res.hex = (&res.hex[res.hex.len() - 1..]).to_string();
            res.bin = (&res.bin[res.bin.len() - 4..]).to_string();
            res
        }
    }
}

#[wasm_bindgen]
impl ResultValue {
    #[wasm_bindgen(getter)]
    pub fn get_signed(&self) -> String {
        self.signed.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_unsigned(&self) -> String {
        self.unsigned.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_bin(&self) -> String {
        self.bin.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn get_hex(&self) -> String {
        self.hex.clone()
    }
}
