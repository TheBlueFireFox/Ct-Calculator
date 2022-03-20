use crate::Supported;

use {
    crate::{
        api::{ResultFlags, ResultValue, Results},
        utils::{self, MAX_I4_U},
    },
    std::convert::TryInto,
};

pub(crate) struct Add;

macro_rules! new {
    ($name:tt, $main_type:ty, $second_type:ty, $parent_type:ty) => {
        fn $name(left: i32, right: i32) -> Results {
            let uleft = left as $main_type;
            let uright = right as $main_type;
            let sleft = uleft as $second_type;
            let sright = uright as $second_type;

            let carry = {
                // build parent type for addition (extended with zeros)
                // so that the carry flag can be detected.
                const SIZEP: usize = std::mem::size_of::<$parent_type>();
                const SIZEM: usize = std::mem::size_of::<$main_type>();

                let transform = |data: $main_type| {
                    let bytes = data.to_be_bytes();

                    let mut parts = [0; SIZEP];

                    parts[(SIZEP - bytes.len())..].copy_from_slice(&bytes);

                    <$parent_type>::from_be_bytes(
                        (&parts[..])
                            .try_into()
                            .expect("This should be sound, as I am doing calculations before."),
                    )
                };

                let left_u16 = transform(uleft);
                let right_u16 = transform(uright);

                let res = left_u16 + right_u16;

                res >> (SIZEM * 8) == 1
            };

            let uresult = uleft.wrapping_add(uright);
            let (sresult, overflow) = sleft.overflowing_add(sright);
            let zero = uresult == 0;
            let negative = sresult < 0;

            let flags = ResultFlags::new(zero, negative, overflow, carry);
            let values = ResultValue::new(uresult, sresult);

            Results::new(flags, values)
        }
    };
}

impl Supported for Add {
    fn new4(left: i32, right: i32) -> Results {
        let cleft = utils::i32_to_u4(left);
        let cright = utils::i32_to_u4(right);

        let tresult = cleft + cright;
        let uresult = utils::to_u4(tresult);
        let sresult = utils::to_i4(uresult);

        let carry = tresult >> 4 == 1;
        let negative = utils::negative(uresult);

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
            // negativee, or you added two negativee numbers and got a positive.
            //
            // If you are doing unsigned arithmetic, the overflow flag means nothing
            // and should be ignored.
            //
            // The rules for two's complement detect errors by examining the sign of
            // the result.  A negativee and positive added together cannot be wrong,
            // because the sum is between the addends. Since both of the addends fit
            // within the allowable range of numbers, and their sum is between them, it
            // must fit as well.  Mixed-sign addition never turns on the overflow flag.
            //
            // In signed arithmetic, watch the overflow flag to detect errors.
            // In unsigned arithmetic, the overflow flag tells you nothing interesting.

            (cright <= MAX_I4_U && cleft <= MAX_I4_U && uresult > MAX_I4_U)
                || (cright > MAX_I4_U && cleft > MAX_I4_U && uresult <= MAX_I4_U)
        };

        let flags = ResultFlags::new(zero, negative, overflow, carry);
        let values = ResultValue::new4(uresult, sresult);

        Results::new(flags, values)
    }

    new!(new8, u8, i8, u16);
    new!(new16, u16, i16, u32);
    new!(new32, u32, i32, u64);
}
