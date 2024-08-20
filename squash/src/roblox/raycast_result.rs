use super::prelude::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct RaycastResult<T: SquashNumber> {
    pub distance: f32,
    pub position: Vector3<T>,
    pub normal: Vector3<T>,
    pub material: EnumItem,
}