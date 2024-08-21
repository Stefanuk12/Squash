use crate::{SquashNumber, Zero};

pub trait SquashFloat: SquashNumber {}
impl SquashFloat for f32 {}
impl SquashFloat for f64 {}
impl Zero for f32 {
    const ZERO: Self = 0.0;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}
