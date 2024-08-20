use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Rect<T: SquashNumber> {
    pub max: Vector2<T>,
    pub min: Vector2<T>,
}
impl_squash!(Rect<T: SquashNumber>, max, min;min, max);