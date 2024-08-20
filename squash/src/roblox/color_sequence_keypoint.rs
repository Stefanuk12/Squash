use super::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct ColorSequenceKeypoint {
    pub value: Color3,
    pub time: u8,
}