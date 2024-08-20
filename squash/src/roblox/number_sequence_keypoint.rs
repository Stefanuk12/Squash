use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct NumberSequenceKeypoint<T: SquashNumber> {
    pub value: T,
    pub envelope: T,
    pub time: u8,
}
impl_squash!(NumberSequenceKeypoint<T: SquashNumber>, value, envelope, time;time, envelope, value);