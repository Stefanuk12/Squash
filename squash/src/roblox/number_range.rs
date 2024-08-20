use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct NumberRange<T: SquashNumber> {
    pub min: T,
    pub max: T
}