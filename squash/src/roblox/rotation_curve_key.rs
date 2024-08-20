use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct RotationCurveKey<T: SquashNumber> {
    pub interpolation: EnumItem,
    pub value: T,
    pub time: f32,
}