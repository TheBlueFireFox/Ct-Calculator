mod utils;

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

#[wasm_bindgen]
pub fn sub(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    let right = !right + 1;

    add(left, right, of)
}

#[wasm_bindgen]
pub fn add(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    match of {
        4 => Ok(addition::new4(left, right)),
        8 => Ok(addition::new8(left, right)),
        16 => Ok(addition::new16(left, right)),
        32 => Ok(addition::new32(left, right)),
        _ => Err(JsValue::from("unsupported value")),
    }
}

const NIBBLE_U8: u8 = 0xF;
const MAX_I4_U: u8 = 7;

pub fn to_i4(val: u8) -> i8 {
    let val = val & NIBBLE_U8;
    let res = if val > MAX_I4_U {
        (NIBBLE_U8 << 4) | val
    } else {
        val
    };
    res as i8
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

                        parts[(size - bytes.len())..].copy_from_slice(&bytes);

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
                let uresult = ileft.wrapping_add(iright);
                let (sresult, overflow) = (ileft as $second_type).overflowing_add(iright as $second_type);
                let zero = uresult == 0;
                let negativ = uresult > (<$second_type>::MAX as $main_type);
                let flags = ResultFlags::new(zero, negativ, overflow, carry);
                let values = ResultValue::new(uresult, sresult);
                Results::new(flags, values)
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
            // see http://teaching.idallen.com/dat2343/10f/notes/040_overflow.txt
            // Overflow Flag
            // -------------
            //
            // The rules for turning on the overflow flag in binary/integer math are two:
            //
            // 1. If the sum of two numbers with the sign bits off yields a result number
            //    with the sign bit on, the "overflow" flag is turned on.
            //
            //    0100 + 0100 = 1000 (overflow flag is turned on)
            //
            // 2. If the sum of two numbers with the sign bits on yields a result number
            //    with the sign bit off, the "overflow" flag is turned on.
            //
            //    1000 + 1000 = 0000 (overflow flag is turned on)
            //
            // Otherwise, the overflow flag is turned off.
            //  * 0100 + 0001 = 0101 (overflow flag is turned off)
            //  * 0110 + 1001 = 1111 (overflow flag is turned off)
            //  * 1000 + 0001 = 1001 (overflow flag is turned off)
            //  * 1100 + 1100 = 1000 (overflow flag is turned off)
            //
            // Note that you only need to look at the sign bits (leftmost) of the three
            // numbers to decide if the overflow flag is turned on or off.
            //
            // If you are doing two's complement (signed) arithmetic, overflow flag on
            // means the answer is wrong - you added two positive numbers and got a
            // negative, or you added two negative numbers and got a positive.
            //
            // If you are doing unsigned arithmetic, the overflow flag means nothing
            // and should be ignored.
            //
            // The rules for two's complement detect errors by examining the sign of
            // the result.  A negative and positive added together cannot be wrong,
            // because the sum is between the addends. Since both of the addends fit
            // within the allowable range of numbers, and their sum is between them, it
            // must fit as well.  Mixed-sign addition never turns on the overflow flag.
            //
            // In signed arithmetic, watch the overflow flag to detect errors.
            // In unsigned arithmetic, the overflow flag tells you nothing interesting.

            (cright <= MAX_I4_U && cleft <= MAX_I4_U && uresult > MAX_I4_U)
                || (cright > MAX_I4_U && cleft > MAX_I4_U && uresult <= MAX_I4_U)
        };

        let flags = ResultFlags::new(zero, negativ, overflow, carry);
        let values = ResultValue::new4(uresult, sresult);

        Results::new(flags, values)
    }

    new!(new8, u8, i8, u16);
    new!(new16, u16, i16, u32);
    new!(new32, u32, i32, u64);
}

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
    pub fn new(zero: bool, negativ: bool, overflow: bool, carry: bool) -> Self {
        Self {
            zero,
            negativ,
            overflow,
            carry,
            borrow: !carry,
        }
    }
}

mod formatter {
    use super::ResultValue;
    use std::fmt::{Binary, Display, UpperHex};

    impl ResultValue {
        pub fn new<U, S>(unsigned: U, signed: S) -> Self
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

        pub fn new4(unsigned: u8, signed: i8) -> Self {
            let mut res = Self::new(unsigned, signed);
            res.hex = (&res.hex[res.hex.len() - 1..]).to_string();
            res.bin = (&res.bin[res.bin.len() - 4..]).to_string();
            res
        }

        fn fix_size<T>(s: String, mult: usize) -> String {
            let size = std::mem::size_of::<T>() * mult;
            format!("{}{}", "0".repeat(size - s.len()), s)
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
