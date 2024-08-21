use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct NumberSequenceKeypoint<T: SquashFloat> {
    pub value: T,
    pub envelope: T,
    pub time: u8,
}
impl_squash!(NumberSequenceKeypoint<T: SquashFloat>, value, envelope, time;time, envelope, value);