use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct RotationCurveKey<T: SquashFloat> {
    pub interpolation: EnumItem,
    pub value: T,
    pub time: f32,
}
impl_squash!(RotationCurveKey<T: SquashFloat>, interpolation, value, time;time, value, interpolation);