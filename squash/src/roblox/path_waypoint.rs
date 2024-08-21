use super::prelude::*;

#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
pub struct PathWaypoint<T: SquashFloat> {
    pub label: String,
    pub action: EnumItem,
    pub position: Vector3<T>,
}
impl_squash!(PathWaypoint<T: SquashFloat>, label, action, position;position, action, label);