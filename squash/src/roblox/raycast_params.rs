use super::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct RaycastParams {
   pub respect_can_collide: bool,
   pub filter_type: EnumItem,
   pub collision_group: String, 
}