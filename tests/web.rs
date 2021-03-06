//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use ct_calculator::{
    api::{ResultFlags, ResultValue, Results},
    utils,
};

#[wasm_bindgen_test]
fn test_human_error() {
    for i in [4, 8, 16, 32].iter() {
        assert_eq!(true, ct_calculator::add(0, 0, *i).is_ok());
    }

    for i in [3, 64].iter() {
        assert_eq!(true, ct_calculator::add(0, 0, *i).is_err());
    }
}

#[wasm_bindgen_test]
fn test_i4() {
    for i in 0..=7 {
        assert_eq!(i as i8, utils::to_i4(i));
    }

    for i in 8..=15 {
        assert_eq!(-((16 - i) as i8), utils::to_i4(i));
    }
}

#[wasm_bindgen_test]
fn test_example() {
    let left = 0x8;
    let right = 0xB;
    {
        let res =ct_calculator::add(left, right, 4).unwrap();
        assert_eq!(true, res.get_flags().overflow);
        assert_eq!(true, res.get_flags().carry);
    }

    {
        let res = ct_calculator::sub(left, right, 4).unwrap();
        assert_eq!(false, res.get_flags().overflow);
        assert_eq!(true, res.get_flags().borrow);
    }
}

#[wasm_bindgen_test]
fn test_add_overflow_no_carry() {
    let left = 0b0110;
    let right = 0b0111;
    let of = 4;
    let flags = ResultFlags::new(false, true, true, false);
    let values = ResultValue::new4(13, -3);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::add);
}

#[wasm_bindgen_test]
fn test_add_overflow_zero() {
    let left = 0b1111;
    let right = 0b0001;
    let of = 4;
    let flags = ResultFlags::new(true, false, false, true);
    let values = ResultValue::new4(0, 0);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::add);
}

#[wasm_bindgen_test]
fn test_add_no_overflow_carry() {
    let left = 0b0111;
    let right = 0b1110;
    let of = 4;
    let flags = ResultFlags::new(false, false, false, true);
    let values = ResultValue::new4(5, 5);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::add);
}

#[wasm_bindgen_test]
fn test_sub_borrow_no_overflow() {
    let left = 0b0110;
    let right = 0b0111;
    let of = 4;
    let flags = ResultFlags::new(false, true, false, false);
    let values = ResultValue::new4(15, -1);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::sub);
}

#[wasm_bindgen_test]
fn test_sub_no_overflow_carry() {
    let left = 0b1111;
    let right = 0b0001;
    let of = 4;
    let flags = ResultFlags::new(false, true, false, true);
    let values = ResultValue::new4(14, -2);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::sub);
}

#[wasm_bindgen_test]
fn test_sub_no_overflow_carry2() {
    let left = 0b1100;
    let right = 0b1011;
    let of = 4;
    let flags = ResultFlags::new(false, false, false, true);
    let values = ResultValue::new4(1, 1);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::sub);
}

#[wasm_bindgen_test]
fn test_sub_overflow_no_carry() {
    let left = 0b0111;
    let right = 0b1110;
    let of = 4;
    let flags = ResultFlags::new(false, true, true, false);
    let values = ResultValue::new4(9, -7);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, ct_calculator::sub);
}

#[wasm_bindgen_test]
fn test_8_one() {
    let left = 0x82;
    let right = 0x12;
    let of = 8;
    {
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, true, false, false);
        let values = ResultValue::new(0x94u8, 0x94u8 as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::add);
    }
    {
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, false, true, true);
        let values = ResultValue::new(0x70u8, 0x70u8 as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::sub);
    }
}

#[wasm_bindgen_test]
fn test_8_two() {
    let left = 0x34;
    let right = 0x72;
    let of = 8;
    {
        let res = 0xA6u8;
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, true, true, false);
        let values = ResultValue::new(res, res as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::add);
    }
    {
        let res = 0xC2u8;
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, true, false, false);
        let values = ResultValue::new(res, res as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::sub);
    }
}

#[wasm_bindgen_test]
fn test_8_three() {
    let left = 0xC2;
    let right = 0x87;
    let of = 8;
    {
        let res = 0x49u8;
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, false, true, true);
        let values = ResultValue::new(res, res as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::add);
    }
    {
        let res = 0x3Bu8;
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, false, false, true);
        let values = ResultValue::new(res, res as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::sub);
    }
}

#[wasm_bindgen_test]
fn test_8_four() {
    let left = 0xA3;
    let right = 0x62;
    let of = 8;
    {
        let res = 0x05u8;
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, false, false, true);
        let values = ResultValue::new(res, res as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::add);
    }
    {
        let res = 0x41u8;
        // zero: bool, negative: bool, overflow: bool, carry: bool
        let flags = ResultFlags::new(false, false, true, true);
        let values = ResultValue::new(res, res as i8);
        let results = Results::new(flags, values);
        testing_facility_results(&results, left, right, of, ct_calculator::sub);
    }
}

fn testing_facility_results<T>(expected: &Results, left: i32, right: i32, of: i32, func: T)
where
    T: FnOnce(i32, i32, i32) -> Result<Results, JsValue>,
{
    let result = func(left, right, of);
    assert_eq!(
        true,
        result.is_ok(),
        "There was an issue while generating the results."
    );

    let result = result.unwrap();

    // check if calculations worked
    // and if hex / binary representations are correct
    let value = result.get_value();
    let expected_value = expected.get_value();

    assert_eq!(
        expected_value.get_signed(),
        value.get_signed(),
        "Signed value wrong"
    );
    assert_eq!(
        expected_value.get_unsigned(),
        value.get_unsigned(),
        "Unsigned value wrong"
    );
    assert_eq!(expected_value.get_bin(), value.get_bin(), "Bin value wrong");
    assert_eq!(expected_value.get_hex(), value.get_hex(), "Hex value wrong");

    // check if flags are correctly set
    let flags = result.get_flags();
    let expected_flags = expected.get_flags();

    assert_eq!(expected_flags.carry, flags.carry, "Carry flag wrong");
    assert_eq!(expected_flags.borrow, flags.borrow, "Borrow flag wrong");
    assert_eq!(
        expected_flags.overflow, flags.overflow,
        "Overflow flag wrong"
    );
    assert_eq!(expected_flags.zero, flags.zero, "Zero flag wrong");
    assert_eq!(
        expected_flags.negative, flags.negative,
        "Negativ flag wrong"
    );
}
