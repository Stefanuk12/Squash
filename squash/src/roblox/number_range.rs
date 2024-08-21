use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct NumberRange<T: SquashFloat> {
    pub min: T,
    pub max: T
}
impl_squash!(NumberRange<T: SquashFloat>, min, max;min, max);