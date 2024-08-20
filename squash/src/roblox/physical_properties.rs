use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct PhysicalProperties {
    pub elasticity_weight: f32,
    pub friction_weight: f32,
    pub elasticity: f32,
    pub friction: f32,
    pub density: f32,
}