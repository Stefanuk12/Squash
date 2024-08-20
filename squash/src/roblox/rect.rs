use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Rect<T: SquashNumber> {
    pub max: Vector2<T>,
    pub min: Vector2<T>,
}