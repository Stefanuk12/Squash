use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Region3<T: SquashNumber> {
    pub size: Vector3<T>,
    pub position: Vector3<T>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Region3int16 {
    pub size: Vector3int16,
    pub position: Vector3int16,
}