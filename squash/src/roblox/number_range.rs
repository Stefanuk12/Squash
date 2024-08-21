use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct NumberRange<T: SquashFloat> {
    pub min: T,
    pub max: T
}
impl_squash!(NumberRange<T: SquashFloat>, min, max;min, max);