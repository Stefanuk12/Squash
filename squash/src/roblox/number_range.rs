use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct NumberRange<T: SquashNumber> {
    pub min: T,
    pub max: T
}
impl_squash!(NumberRange<T: SquashNumber>, min, max;min, max);