use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Udim<T: SquashNumber> {
    pub offset: T,
    pub scale: T,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Udim2<T: SquashNumber> {
    pub y: Udim<T>,
    pub x: Udim<T>,
}