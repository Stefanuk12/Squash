use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize,))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Color3 {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}
impl_squash!(Color3, r, g, b;b, g, r);