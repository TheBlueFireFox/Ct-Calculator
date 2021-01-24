//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use ct_calculator::*;

#[wasm_bindgen_test]
fn test_human_error() {
    for i in [4, 8, 16, 32].iter() {
        assert_eq!(true, add(0, 0, *i).is_ok());
    }

    for i in [3, 64].iter() {
        assert_eq!(true, add(0, 0, *i).is_err());
    }
}

#[wasm_bindgen_test]
fn test_i4() {
    for i in 0..=7 {
        assert_eq!(i as i8, to_i4(i));
    }

    for i in 8..=7 {
        assert_eq!((i - 16) as i8, to_i4(i));
    }
}

#[wasm_bindgen_test]
fn test_add_overflow_no_carry() {
    let result = add(0b0110, 0b0111, 4);
    // check if the value is okay
    assert_eq!(true, result.is_ok());

    let result = result.unwrap();
    let value = result.get_value();
    let flags = result.get_flags();

    // check if calculations worked
    // and if hex / binary representations are correct
    assert_eq!("-3", &value.get_signed()[..]);
    assert_eq!("13", &value.get_unsigned()[..]);
    assert_eq!("1101", &value.get_bin()[..]);
    assert_eq!("D", &value.get_hex()[..]);

    // check if flags are correctly set
    assert_eq!(false, flags.carry);
    assert_eq!(true, flags.borrow);
    assert_eq!(true, flags.overflow);
    assert_eq!(false, flags.zero);
}

#[wasm_bindgen_test]
fn test_add_overflow_zero() {
    let result = add(0b1111, 0b0001, 4);
    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get_value();
    let flags = result.get_flags();

    // check if calculations worked
    // and if hex / binary representations are correct
    assert_eq!("0", &value.get_signed()[..]);
    assert_eq!("0", &value.get_unsigned()[..]);
    assert_eq!("0000", &value.get_bin()[..]);
    assert_eq!("0", &value.get_hex()[..]);

    // check if flags are correctly set
    assert_eq!(true, flags.carry);
    assert_eq!(false, flags.borrow);
    assert_eq!(false, flags.overflow);
    assert_eq!(true, flags.zero);

    let left = 0b1111;
    let right = 0b0001;
    let of = 4;
    let flags = ResultFlags::new(true, false, false, true);
    let values = ResultValue::new4(5, 5);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, add);
}

#[wasm_bindgen_test]
fn test_add_no_overflow_carry() {
    let left = 0b0111;
    let right = 0b1110;
    let of = 4;
    let flags = ResultFlags::new(false, false, false, true);
    let values = ResultValue::new4(5, 5);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, add);
}

#[wasm_bindgen_test]
fn test_sub_borrow_no_overflow() {
    let left = 0b0110;
    let right = 0b0111;
    let of = 4;
    let flags = ResultFlags::new(false, true, false, false);
    let values = ResultValue::new4(15, -1);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, sub);
}

#[wasm_bindgen_test]
fn test_sub_no_overflow_carry() {
    let left = 0b1111;
    let right = 0b0001;
    let of = 4;
    let flags = ResultFlags::new(false, true, false, true);
    let values = ResultValue::new4(14, -2);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, sub);
}

#[wasm_bindgen_test]
fn test_sub_overflow_no_carry() {
    let left = 0b0111;
    let right = 0b1110;
    let of = 4;
    let flags = ResultFlags::new(false, true, true, false);
    let values = ResultValue::new4(9, -7);
    let results = Results::new(flags, values);
    testing_facility_results(&results, left, right, of, sub);
}

fn testing_facility_results<T>(expected: &Results, left: i32, right: i32, of: i32, func: T)
where
    T: FnOnce(i32, i32, i32) -> Result<Results, JsValue>
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
    assert_eq!(expected_flags.negativ, flags.negativ, "Negativ flag wrong");
}
