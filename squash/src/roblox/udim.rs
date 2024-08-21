use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Udim<T: SquashFloat> {
    pub offset: T,
    pub scale: T,
}
impl_squash!(Udim<T: SquashFloat>, offset, scale;scale, offset);

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Udim2<T: SquashFloat> {
    pub y: Udim<T>,
    pub x: Udim<T>,
}
impl_squash!(Udim2<T: SquashFloat>, y, x;x, y);