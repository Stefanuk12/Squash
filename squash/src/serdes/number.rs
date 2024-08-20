use serde::de::DeserializeOwned;

use super::SquashObject;

pub trait Zero {
    const ZERO: Self;
}

pub trait SquashNumber: DeserializeOwned + SquashObject + Clone + Zero {}
impl SquashNumber for f32 {}
impl SquashNumber for f64 {}
impl Zero for f32 {
    const ZERO: Self = 0.0;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}

macro_rules! impl_zero {
    ($($t:ty)*) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0;
            }
        )*
    }
}

impl_zero!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);