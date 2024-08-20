use super::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct PathWaypoint<T: SquashNumber> {
    pub label: String,
    pub action: EnumItem,
    pub position: Vector3<T>,
}