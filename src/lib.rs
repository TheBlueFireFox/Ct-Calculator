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

#[wasm_bindgen]
pub fn sub(left:i32, right: i32, of: i32) -> Result<Results, JsValue> {
    // transform right to it's complement
    let right = !right + 1;

    add(left, right, of)
}

#[wasm_bindgen]
pub fn add(left: i32, right: i32, of: i32) -> Result<Results, JsValue> {
    match of {
        4 => Ok(Results::new4(left, right)),
        8 => Ok(Results::new8(left, right)),
        16 => Ok(Results::new16(left, right)),
        32 => Ok(Results::new32(left, right)),
        _ => Err(JsValue::from("unsupported value")),
    }
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Results {
    flags: ResultFlags,
    values: ResultValue,
}

#[wasm_bindgen]
impl Results {
    fn new4(left: i32, right: i32) -> Self {
        const NIBBLE: u8 = 0xF;
        const MAX_I4: u8 = 7;

        let cleft = left as u8 & NIBBLE;
        let cright = right as u8 & NIBBLE;

        let tresult = cleft + cright;
        let uresult = tresult & NIBBLE;
        let sresult = if uresult > 7 {
            (NIBBLE << 4) | uresult
        } else {
            uresult
        } as i8;

        let carry = tresult >> 4 == 1;
        let negativ = uresult > MAX_I4;
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
           (left >= 0 && right >= 0 && sresult < 0) || (left < 0 && right < 0 && sresult >= 0)
        };

        let flags = ResultFlags::new(zero, negativ, overflow, carry);
        let values = ResultValue::new(uresult, sresult);

        Self { flags, values }
    }

    fn new8(left: i32, right: i32) -> Self {
        let ileft = left as u8;
        let iright = right as u8;
        let carry = {
            // build u16 for addition (extend with zeros)
            let left_u16 = u16::from_be_bytes([0, ileft]);
            let right_u16 = u16::from_be_bytes([0, iright]);
            let res = left_u16 + right_u16;
            res >> 8 == 1
        };
        let (uresult, overflow) = ileft.overflowing_add(iright);
        let zero = uresult == 0;
        let negativ = uresult > (i8::MAX as u8);
        let sresult = uresult as i8;
        let flags = ResultFlags::new(zero, negativ, overflow, carry);
        let values = ResultValue::new(uresult, sresult);
        Self { flags, values }
    }

    fn new16(left: i32, right: i32) -> Self {
        let ileft = left as u16;
        let iright = right as u16;
        let carry = {
            // build u16 for addition (extend with zeros)
            let left_bytes = ileft.to_be_bytes();
            let right_bytes = iright.to_be_bytes();
            let left_u16 = u32::from_be_bytes([0, 0, left_bytes[0], left_bytes[1]]);
            let right_u16 = u32::from_be_bytes([0, 0, right_bytes[0], right_bytes[1]]);
            let res = left_u16 + right_u16;
            res >> 16 == 1
        };

        let (uresult, overflow) = ileft.overflowing_add(iright);
        let zero = uresult == 0;
        let negativ = uresult > (i16::MAX as u16);
        let sresult = uresult as i16;
        let flags = ResultFlags::new(zero, negativ, overflow, carry);
        let values = ResultValue::new(uresult, sresult);
        Self { flags, values }
    }

    fn new32(left: i32, right: i32) -> Self {
        let ileft = left as u32;
        let iright = right as u32;
        let carry = {
            // build u16 for addition (extend with zeros)
            let left_bytes = ileft.to_be_bytes();
            let right_bytes = iright.to_be_bytes();
            let left_u16 = u64::from_be_bytes([
                0,
                0,
                0,
                0,
                left_bytes[0],
                left_bytes[1],
                left_bytes[2],
                left_bytes[3],
            ]);
            let right_u16 = u64::from_be_bytes([
                0,
                0,
                0,
                0,
                right_bytes[0],
                right_bytes[1],
                right_bytes[2],
                right_bytes[3],
            ]);
            let res = left_u16 + right_u16;
            res >> 32 == 1
        };
        let (uresult, overflow) = ileft.overflowing_add(iright);
        let zero = uresult == 0;
        let negativ = uresult > (i32::MAX as u32);
        let sresult = uresult as i32;
        let flags = ResultFlags::new(zero, negativ, overflow, carry);
        let values = ResultValue::new(uresult, sresult);
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