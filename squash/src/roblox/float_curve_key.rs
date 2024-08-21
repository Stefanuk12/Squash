use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct FloatCurveKey {
    pub interpolation: EnumItem,
    pub value: f32,
    pub time: f32,
}
impl_squash!(FloatCurveKey, interpolation, value, time;time, value, interpolation);