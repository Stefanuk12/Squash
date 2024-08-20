use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Udim<T: SquashNumber> {
    pub offset: T,
    pub scale: T,
}
impl_squash!(Udim<T: SquashNumber>, offset, scale;scale, offset);

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Udim2<T: SquashNumber> {
    pub y: Udim<T>,
    pub x: Udim<T>,
}
impl_squash!(Udim2<T: SquashNumber>, y, x;x, y);