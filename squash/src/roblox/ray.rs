use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Ray<T: SquashNumber> {
    pub direction: Vector3<T>,
    pub origin: Vector3<T>,
}
impl_squash!(Ray<T: SquashNumber>, direction, origin;origin, direction);