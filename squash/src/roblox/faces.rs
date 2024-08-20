use super::prelude::*;

pub struct Faces {
    pub back: bool,
    pub bottom: bool,
    pub front: bool,
    pub left: bool,
    pub right: bool,
    pub top: bool,
}
impl Serialize for Faces {
    fn serialize<S>(&self, serializer: S) -> CoreResult<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_u8(
            (self.back as u8) |
            (self.bottom as u8) << 1 |
            (self.front as u8) << 2 |
            (self.left as u8) << 3 |
            (self.right as u8) << 4 |
            (self.top as u8) << 5
        )
    }
}
impl<'de> Deserialize<'de> for Faces {
    fn deserialize<D>(deserializer: D) -> CoreResult<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let x = u8::deserialize(deserializer)?;
        Ok(Faces {
            back: (x & 1) != 0,
            bottom: (x & 2) != 0,
            front: (x & 4) != 0,
            left: (x & 8) != 0,
            right: (x & 16) != 0,
            top: (x & 32) != 0,
        })
    }
}