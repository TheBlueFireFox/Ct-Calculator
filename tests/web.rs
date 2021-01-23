//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

use ct_calculator::*;

#[wasm_bindgen_test]
fn test_human_error() {
    for i in [4, 8, 16, 32].iter() {
        assert_eq!(true, add(0, 0, *i).is_ok())
    }

    for i in [3, 64].iter() {
        assert_eq!(true, add(0, 0, *i).is_err())
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
}

#[wasm_bindgen_test]
fn test_add_no_overflow_carry() {
    let result = add(0b0111, 0b1110, 4);
    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get_value();
    let flags = result.get_flags();

    // check if calculations worked
    // and if hex / binary representations are correct
    assert_eq!("5", &value.get_signed()[..]);
    assert_eq!("5", &value.get_unsigned()[..]);
    assert_eq!("0101", &value.get_bin()[..]);
    assert_eq!("5", &value.get_hex()[..]);

    // check if flags are correctly set
    assert_eq!(true, flags.carry);
    assert_eq!(false, flags.borrow);
    assert_eq!(false, flags.overflow);
    assert_eq!(false, flags.zero);
}

#[wasm_bindgen_test]
fn test_sub_borrow_no_overflow() {
    let result = sub(0b0110, 0b0111, 4);
    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get_value();
    let flags = result.get_flags();

    // check if calculations worked
    // and if hex / binary representations are correct
    assert_eq!("-1", &value.get_signed()[..]);
    assert_eq!("15", &value.get_unsigned()[..]);
    assert_eq!("1111", &value.get_bin()[..]);
    assert_eq!("F", &value.get_hex()[..]);

    // check if flags are correctly set
    assert_eq!(false, flags.carry);
    assert_eq!(true, flags.borrow);
    assert_eq!(false, flags.overflow);
    assert_eq!(false, flags.zero);
}

#[wasm_bindgen_test]
fn test_sub_no_overflow_carry() {
    let result = sub(0b1111, 0b0001, 4);
    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get_value();
    let flags = result.get_flags();

    // check if calculations worked
    // and if hex / binary representations are correct
    assert_eq!("-2", &value.get_signed()[..]);
    assert_eq!("14", &value.get_unsigned()[..]);
    assert_eq!("1110", &value.get_bin()[..]);
    assert_eq!("E", &value.get_hex()[..]);

    // check if flags are correctly set
    assert_eq!(true, flags.carry);
    assert_eq!(false, flags.borrow);
    assert_eq!(false, flags.overflow);
    assert_eq!(false, flags.zero);
}

#[wasm_bindgen_test]
fn test_sub_overflow_no_carry() {
    let result = sub(0b0111, 0b1110, 4);
    assert!(result.is_ok());

    let result = result.unwrap();
    let value = result.get_value();
    let flags = result.get_flags();

    // check if calculations worked
    // and if hex / binary representations are correct
    assert_eq!("-7", &value.get_signed()[..]);
    assert_eq!("9", &value.get_unsigned()[..]);
    assert_eq!("1001", &value.get_bin()[..]);
    assert_eq!("9", &value.get_hex()[..]);

    // check if flags are correctly set
    assert_eq!(false, flags.carry);
    assert_eq!(true, flags.borrow);
    assert_eq!(true, flags.overflow);
    assert_eq!(false, flags.zero);
}
