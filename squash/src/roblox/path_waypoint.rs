use super::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct PathWaypoint<T: SquashNumber> {
    pub label: String,
    pub action: EnumItem,
    pub position: Vector3<T>,
}
impl_squash!(PathWaypoint<T: SquashNumber>, label, action, position;position, action, label);