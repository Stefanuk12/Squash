use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Ray<T: SquashNumber> {
    pub direction: Vector3<T>,
    pub origin: Vector3<T>,
}