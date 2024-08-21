use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Serialize)]
pub struct Vector3<T: SquashFloat> {
    pub z: T,
    pub y: T,
    pub x: T,
}
impl_squash!(Vector3<T: SquashFloat>, x, y, z; z, y, x);
impl<T: SquashFloat> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl<T: SquashFloat> Zero for Vector3<T> {
    const ZERO: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO };
}
impl<T: SquashFloat> Default for Vector3<T> {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub struct Vector3int16 {
    pub z: i16, 
    pub y: i16,
    pub x: i16,
}
impl_squash!(Vector3int16, x, y, z; z, y, x);