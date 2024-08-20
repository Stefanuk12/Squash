use super::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct RaycastParams {
   pub respect_can_collide: bool,
   pub filter_type: EnumItem,
   pub collision_group: String, 
}
impl_squash!(RaycastParams, respect_can_collide, filter_type, collision_group;collision_group, filter_type, respect_can_collide);