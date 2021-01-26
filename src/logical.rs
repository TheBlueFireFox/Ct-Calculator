use std::ops::{BitAnd, BitOr, BitXor};

use crate::{
    api::{ResultFlags, ResultValue},
    utils, Results, Supported,
};

macro_rules! working {
    ($name:tt, $main_type:ty, $second_type:ty) => {
        fn $name(left: i32, right: i32) -> Results {
            let left = left as $main_type;
            let right = right as $main_type;

            // result
            let ures = Self::run(left, right);
            let sres = ures as $second_type;

            // flags
            let zero = ures == 0;
            let negative = sres < 0;

            let results = ResultValue::new(ures, sres);
            let flags = ResultFlags::new(zero, negative, false, false);

            Results::new(flags, results)
        }
    };
}

macro_rules! workingu4 {
    () => {
        fn new4(left: i32, right: i32) -> Results {
            let left = utils::i32_to_u4(left);
            let right = utils::i32_to_u4(right);

            // result
            let ures = Self::run(left, right);
            let sres = utils::to_i4(ures);

            // flags
            let zero = ures == 0;
            let negative = utils::negative(ures);

            let results = ResultValue::new4(ures, sres);
            let flags = ResultFlags::new(zero, negative, false, false);

            Results::new(flags, results)
        }
    };
}

macro_rules! functs {
    ($name:ident) => {
        impl Supported for $name {
            workingu4!();
            working!(new8, u8, i8);
            working!(new16, u16, i16);
            working!(new32, u32, i32);
        }
    };
}

pub struct AND;
trait DoWork {
    fn run<T>(left: T, right: T) -> T
    where
        T: num::Unsigned + BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>;
}

impl DoWork for AND {
    fn run<T>(left: T, right: T) -> T
    where
        T: num::Unsigned + BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>,
    {
        left & right
    }
}

functs!(AND);

pub struct OR;

impl DoWork for OR {
    fn run<T>(left: T, right: T) -> T
    where
        T: num::Unsigned + BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>,
    {
        left | right
    }
}

functs!(OR);

pub struct XOR;

impl DoWork for XOR {
    fn run<T>(left: T, right: T) -> T
    where
        T: num::Unsigned + BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T>,
    {
        left ^ right
    }
}

functs!(XOR);
