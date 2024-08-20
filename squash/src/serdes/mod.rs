use std::io::Cursor;

use derive_more::{From, Into, FromStr, AsRef, AsMut, Index, Deref, Not, Add, Mul, Sum, IndexMut, DerefMut, AddAssign, MulAssign};
use serde::{Deserialize, Serialize};

import!(ser, des, cursor, bool, number, uint, int, ux);

#[derive(From, Into, FromStr, AsRef, AsMut, derive_more::Debug, derive_more::Display, Index, Deref, Not, Add, Mul, Sum, IndexMut, DerefMut, AddAssign, MulAssign)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Vlq(pub u64);
impl Serialize for Vlq {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut cursor = Cursor::new(Vec::new());
        cursor.push(*self).map_err(|e| serde::ser::Error::custom(e))?;
        serializer.serialize_bytes(cursor.into_inner().as_slice())
    }
}
impl<'de> Deserialize<'de> for Vlq {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        let mut cursor = Cursor::new(bytes);
        cursor.pop::<Vlq>().map_err(|e| serde::de::Error::custom(e))
    }
}