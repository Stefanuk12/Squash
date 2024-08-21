use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct RaycastResult<T: SquashFloat> {
    pub distance: f32,
    pub position: Vector3<T>,
    pub normal: Vector3<T>,
    pub material: EnumItem,
}
impl_squash!(RaycastResult<T: SquashFloat>, distance, position, normal, material;material, normal, position, distance);