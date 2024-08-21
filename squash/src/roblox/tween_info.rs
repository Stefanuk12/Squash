use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct TweenInfo {
    pub delay_time: f32,
    pub reverses: bool,
    pub repeat_count: Vlq,
    pub easing_direction: EnumItem,
    pub easing_style: EnumItem,
    pub time: f32,
}
impl_squash!(TweenInfo, delay_time, reverses, repeat_count, easing_direction, easing_style, time;time, easing_style, easing_direction, repeat_count, reverses, delay_time);