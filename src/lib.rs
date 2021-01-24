mod utils;

use std::fmt::{Binary, Display, UpperHex};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, ct-calculator!");
}

macro_rules! func {
    ($name:tt, $path:tt) => {
        #[wasm_bindgen]
        pub fn $name(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
            match of {
                4 => Ok($path::new4(left, right)),
                8 => Ok($path::new8(left, right)),
                16 => Ok($path::new16(left, right)),
                32 => Ok($path::new32(left, right)),
                _ => Err(JsValue::from("unsupported value")),
            }
        }
    };
}

func!(sub, subtraction);

func!(add, addition);

#[wasm_bindgen]
#[derive(Debug)]
pub struct Results {
    flags: ResultFlags,
    values: ResultValue,
}

const NIBBLE_U8: u8 = 0xF;
const MAX_I4_U: u8 = 7;
const MAX_I4_I: i8 = 7;
const MIN_I4_I: i8 = -8;

fn to_i4(val: u8) -> i8 {
    let val = val & NIBBLE_U8;
    let res = if val > MAX_I4_U {
        (NIBBLE_U8 << 4) | val
    } else {
        val
    };
    res as i8
}

mod subtraction {
    use super::*;

    pub(crate) fn new4(left: i32, right: i32) -> Results {
        // transform right to it's complement
        let right = !right + 1;

        let mut res = addition::new4(left, right);
        let mut flags = res.get_mut_flags();
        // ((si_b > 0 && si_a < INT_MIN + si_b) ||
        // (si_b < 0 && si_a > INT_MAX + si_b))

        let cleft = to_i4(left as u8 & NIBBLE_U8);
        let cright = to_i4(right as u8 & NIBBLE_U8);

        flags.overflow = {
            (cright > 0 && cleft < (MIN_I4_I + cright))
                || (cright < 0 && cleft > (MAX_I4_I + cright))
        };
        res
    }

    macro_rules! new {
        ($name:tt, $type:ty) => {
            pub(crate) fn $name(left: i32, right: i32) -> Results {
                // transform right to it's complement
                let right = !right + 1;

                let mut res = addition::$name(left, right);
                let mut flags = res.get_mut_flags();
                let ileft = left as $type;
                let iright = right as $type;
                let (_, overflow) = ileft.overflowing_sub(iright);
                flags.overflow = overflow;
                res
            }
        };
    }

    new!(new8, i8);
    new!(new16, i16);
    new!(new32, i32);
}

mod addition {
    use std::convert::TryInto;

    use super::*;

    macro_rules! new {
        ($name:tt, $main_type:ty, $second_type:ty, $parent_type:ty) => {
            pub(crate) fn $name(left: i32, right: i32) -> Results {
                let ileft = left as $main_type;
                let iright = right as $main_type;
                let carry = {
                    // build u16 for addition (extend with zeros)

                    let transform = |data: $main_type| {
                        let bytes = data.to_be_bytes();

                        let mut parts = [0; std::mem::size_of::<$parent_type>()];

                        let size = std::mem::size_of::<$parent_type>();

                        parts[(size - bytes.len() - 1)..].copy_from_slice(&bytes);

                        <$parent_type>::from_be_bytes(
                            (&parts[..])
                                .try_into()
                                .expect("This should be sound, as I am doing calculations before."),
                        )
                    };

                    let left_u16 = transform(ileft);
                    let right_u16 = transform(iright);

                    let res = left_u16 + right_u16;
                    res >> (std::mem::size_of::<$main_type>() * 8) == 1
                };
                let (uresult, overflow) = ileft.overflowing_add(iright);
                let zero = uresult == 0;
                let negativ = uresult > (<$second_type>::MAX as $main_type);
                let sresult = uresult as $second_type;
                let flags = ResultFlags::new(zero, negativ, overflow, carry);
                let values = ResultValue::new(uresult, sresult);
                Results { flags, values }
            }
        };
    }

    pub(crate) fn new4(left: i32, right: i32) -> Results {
        let cleft = left as u8 & NIBBLE_U8;
        let cright = right as u8 & NIBBLE_U8;

        let tresult = cleft + cright;
        let uresult = tresult & NIBBLE_U8;
        let sresult = to_i4(uresult);

        let carry = tresult >> 4 == 1;
        let negativ = uresult > MAX_I4_U;
        let zero = uresult == 0;

        let overflow = {
            // 1)  Calculate sum
            // 2)  Check for conditions
            //     If both numbers are positive and sum is negative then
            //        return true
            //     Else If both numbers are negative and sum is positive then
            //        return true
            //     Else
            //        return false
            // works only for addition :-(
            (left >= 0 && right >= 0 && sresult < 0) || (left < 0 && right < 0 && sresult >= 0)

            // doesn't work
            // const MAX_I4_S: i8 = 0x7;
            // const MIN_I4_S: i8 = 0xF;
            // const NIBBLE_S : i8 = 0xF;
            // let si_b = right as i8 & NIBBLE_S;
            // let si_a = left as i8 & NIBBLE_S;
            // (si_b > 0 && si_a < MIN_I4_S + si_b)
            //     || (si_b < 0) && (si_a > MAX_I4_S + si_b)
        };

        let flags = ResultFlags::new(zero, negativ, overflow, carry);
        let mut values = ResultValue::new(uresult, sresult);

        values.hex = (&values.hex[values.hex.len() - 1..]).to_string();
        values.bin = (&values.bin[values.bin.len() - 4..]).to_string();

        Results { flags, values }
    }

    new!(new8, u8, i8, u16);
    new!(new16, u16, i16, u32);
    new!(new32, u32, i32, u64);
}

#[wasm_bindgen]
impl Results {
    pub(crate) fn get_mut_flags(&mut self) -> &mut ResultFlags {
        &mut self.flags
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
#[derive(Debug, Clone, Copy)]
pub struct ResultFlags {
    pub zero: bool,
    pub negativ: bool,
    pub overflow: bool,
    pub carry: bool,
    pub borrow: bool,
}

#[wasm_bindgen]
impl ResultFlags {
    fn new(zero: bool, negativ: bool, overflow: bool, carry: bool) -> Self {
        Self {
            zero,
            negativ,
            overflow,
            carry,
            borrow: !carry,
        }
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

#[wasm_bindgen]
impl ResultValue {
    fn new<U, S>(unsigned: U, signed: S) -> Self
    where
        U: num::Unsigned + Display + UpperHex + Binary,
        S: num::Signed + Display,
    {
        Self {
            unsigned: format!("{}", unsigned),
            signed: format!("{}", signed),
            hex: ResultValue::fix_size::<U>(format!("{:X}", unsigned), 2),
            bin: ResultValue::fix_size::<U>(format!("{:b}", unsigned), 8),
        }
    }

    fn fix_size<T>(s: String, mult: usize) -> String {
        let size = std::mem::size_of::<T>() * mult;
        format!("{}{}", "0".repeat(size - s.len()), s)
    }

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
