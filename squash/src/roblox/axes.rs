use super::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Axes {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
    pub back: bool,
    pub front: bool,
}
impl Serialize for Axes {
    fn serialize<S>(&self, serializer: S) -> CoreResult<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_u16(
            (self.back as u16) |
            (self.bottom as u16) << 1 |
            (self.front as u16) << 2 |
            (self.left as u16) << 3 |
            (self.right as u16) << 4 |
            (self.top as u16) << 5 |
            (self.x as u16) << 8 |
            (self.y as u16) << 9 |
            (self.z as u16) << 10
        )
    }
}
impl<'de> Deserialize<'de> for Axes {
    fn deserialize<D>(deserializer: D) -> CoreResult<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let x = u16::deserialize(deserializer)?;
        Ok(Axes {
            back: (x & 1) != 0,
            front: (x & 2) != 0,
            bottom: (x & 4) != 0,
            left: (x & 8) != 0,
            right: (x & 16) != 0,
            top: (x & 32) != 0,
            x: (x & 256) != 0,
            y: (x & 512) != 0,
            z: (x & 1024) != 0,
        })
    }
}