use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Rect<T: SquashFloat> {
    pub max: Vector2<T>,
    pub min: Vector2<T>,
}
impl_squash!(Rect<T: SquashFloat>, max, min;min, max);