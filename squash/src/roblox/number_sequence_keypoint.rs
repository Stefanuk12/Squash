use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct NumberSequenceKeypoint<T: SquashNumber> {
    pub value: T,
    pub envelope: T,
    pub time: u8,
}