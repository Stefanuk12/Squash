use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct Rect<T: SquashFloat> {
    pub max: Vector2<T>,
    pub min: Vector2<T>,
}
impl_squash!(Rect<T: SquashFloat>, max, min;min, max);