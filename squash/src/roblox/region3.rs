use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Region3<T: SquashFloat> {
    pub size: Vector3<T>,
    pub position: Vector3<T>,
}
impl_squash!(Region3<T: SquashFloat>, size, position;position, size);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub struct Region3int16 {
    pub size: Vector3int16,
    pub position: Vector3int16,
}
impl_squash!(Region3int16, size, position;position, size);