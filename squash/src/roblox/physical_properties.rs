use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct PhysicalProperties {
    pub elasticity_weight: f32,
    pub friction_weight: f32,
    pub elasticity: f32,
    pub friction: f32,
    pub density: f32,
}
impl_squash!(PhysicalProperties, elasticity_weight, friction_weight, elasticity, friction, density;density, friction, elasticity, friction_weight, elasticity_weight);