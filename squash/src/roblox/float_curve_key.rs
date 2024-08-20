use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct FloatCurveKey {
    pub interpolation: EnumItem,
    pub value: f32,
    pub time: f32,
}