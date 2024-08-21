use crate::BoolTuple2;

use super::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct OverlapParamsBool {
    pub brute_force_all_slow: bool,
    pub respect_can_collide: bool
}

#[cfg(feature = "serde")]
impl Serialize for OverlapParamsBool {
    fn serialize<S>(&self, serializer: S) -> CoreResult<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("OverlapParamsBool", 2)?;
        state.serialize_field("brute_force_all_slow", &self.brute_force_all_slow)?;
        state.serialize_field("respect_can_collide", &self.respect_can_collide)?;
        state.end()
    }
}
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for OverlapParamsBool {
    fn deserialize<D>(deserializer: D) -> CoreResult<Self, D::Error>
        where
            D: Deserializer<'de> {
        let x = u8::deserialize(deserializer)?;
        Ok(OverlapParamsBool {
            brute_force_all_slow: (x & 1) != 0,
            respect_can_collide: (x & 2) != 0
        })
    }
}

impl SquashObject for OverlapParamsBool {
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: crate::SquashCursor,
                Self: Sized {
        BoolTuple2::pop_obj(cursor).map(|x| OverlapParamsBool { brute_force_all_slow: x.0, respect_can_collide: x.1 })
    }
    fn push_obj<T: crate::SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        let x = BoolTuple2(self.brute_force_all_slow, self.respect_can_collide);
        cursor.push(x)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct OverlapParams {
    pub bool_data: OverlapParamsBool,
    pub max_parts: u16,
    pub raycast_filter_type: EnumItem,
    pub collision_group: String
}
impl_squash!(OverlapParams, bool_data, max_parts, raycast_filter_type, collision_group;collision_group, raycast_filter_type, max_parts, bool_data);