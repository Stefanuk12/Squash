use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Serialize, ReverseDeserialize, SquashObject)]
pub struct Vector3<T: SquashNumber> {
    pub z: T,
    pub y: T,
    pub x: T,
}
impl<T: SquashNumber> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}
impl<T: SquashNumber> Zero for Vector3<T> {
    const ZERO: Self = Self { x: T::ZERO, y: T::ZERO, z: T::ZERO };
}
impl<T: SquashNumber> Default for Vector3<T> {
    fn default() -> Self {
        Self::ZERO
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Vector3int16 {
    pub z: i16, 
    pub y: i16,
    pub x: i16,
}