use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct TweenInfo {
    pub delay_time: f32,
    pub reverses: bool,
    pub repeat_count: Vlq,
    pub easing_direction: EnumItem,
    pub easing_style: EnumItem,
    pub time: f32,
}