use super::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Color3 {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}