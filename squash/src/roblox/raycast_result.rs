use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct RaycastResult<T: SquashNumber> {
    pub distance: f32,
    pub position: Vector3<T>,
    pub normal: Vector3<T>,
    pub material: EnumItem,
}
impl_squash!(RaycastResult<T: SquashNumber>, distance, position, normal, material;material, normal, position, distance);