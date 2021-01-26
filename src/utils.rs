#[allow(dead_code)]
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub const NIBBLE_U8: u8 = 0xF;
pub const MAX_I4_U: u8 = 7;

type I4 = i8;
type U4 = u8;

pub fn to_u4(val: u8) -> U4 {
    val & NIBBLE_U8
}

pub fn i32_to_u4(val: i32) -> U4 {
    (val as u8) & NIBBLE_U8
}

pub fn negative(val: u8) -> bool {
    val > MAX_I4_U
}

pub fn to_i4(val: u8) -> I4 {
    let val = val & NIBBLE_U8;
    let res = if val > MAX_I4_U {
        (NIBBLE_U8 << 4) | val
    } else {
        val
    };
    res as i8
}
