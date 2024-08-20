use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct RotationCurveKey<T: SquashNumber> {
    pub interpolation: EnumItem,
    pub value: T,
    pub time: f32,
}
impl_squash!(RotationCurveKey<T: SquashNumber>, interpolation, value, time;time, value, interpolation);