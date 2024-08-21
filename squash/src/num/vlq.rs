use std::io::Cursor;

use ux::*;

use crate::prelude::*;

#[derive(From, Into, FromStr, AsRef, AsMut, derive_more::Debug, derive_more::Display, Index, Deref, Not, Add, Mul, Sum, IndexMut, DerefMut, AddAssign, MulAssign)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Vlq(pub u64);
impl SquashObject for Vlq {
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: SquashCursor,
                Self: Sized {
        let mut x = 0;
        let mut counter = 0;

        for _ in 0..=7 {
            let b = cursor.pop::<u8>()? as u64;
            if b >= 128 {
                return Ok(Self(b as u64 - 128) * 128_u64.pow(counter));
            }

            counter += 1;

            let b = cursor.pop::<u8>()? as u64;
            x += b * 128_u64.pow(counter);
        }
        
        Err(crate::Error::InvalidVlq(x))
    }
    fn push_obj<T: SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        let value = self.0;
        let x0 = value % 128;
        let x1 = value / 128 % 128;
        let x2 = value / 128 / 128 % 128;
        let x3 = value / 128 / 128 / 128 % 128;
        let x4 = value / 128 / 128 / 128 / 128 % 128;
        let x5 = value / 128 / 128 / 128 / 128 / 128 % 128;
        let x6 = value / 128 / 128 / 128 / 128 / 128 / 128 % 128;
        let x7 = value / 128 / 128 / 128 / 128 / 128 / 128 / 128 % 128;

        if x7 != 0 {
            let y = x0 * 256_u64.pow(7) + x1 * 256_u64.pow(6) + x2 * 256_u64.pow(5) + x3 * 256_u64.pow(4) + x4 * 256_u64.pow(3) + x5 * 256_u64.pow(2) + x6 * 256 + x7 + 128;
            cursor.push(y)
        } else if x6 != 0 {
            let y = u56::try_from(x0 * 256_u64.pow(6) + x1 * 256_u64.pow(5) + x2 * 256_u64.pow(4) + x3 * 256_u64.pow(3) + x4 * 256_u64.pow(2) + x5 * 256 + x6 + 128).unwrap();
            cursor.push(y)
        } else if x5 != 0 {
            let y = u48::try_from(x0 * 256_u64.pow(5) + x1 * 256_u64.pow(4) + x2 * 256_u64.pow(3) + x3 * 256_u64.pow(2) + x4 * 256 + x5 + 128).unwrap();
            cursor.push(y)
        } else if x4 != 0 {
            let y = u40::try_from(x0 * 256_u64.pow(4) + x1 * 256_u64.pow(3) + x2 * 256_u64.pow(2) + x3 * 256 + x4 + 128).unwrap();
            cursor.push(y)
        } else if x3 != 0 {
            let y = u32::try_from(x0 * 256_u64.pow(3) + x1 * 256_u64.pow(2) + x2 * 256 + x3 + 128).unwrap();
            cursor.push(y)
        } else if x2 != 0 {
            let y = u24::try_from(x0 * 256_u64.pow(2) + x1 * 256 + x2 + 128).unwrap();
            cursor.push(y)
        } else if x1 != 0 {
            let y = u16::try_from(x0 * 256 + x1 + 128).unwrap();
            cursor.push(y)
        } else {
            let y = u8::try_from(x0 + 128).unwrap();
            cursor.push(y)
        }
    }
}

impl Serialize for Vlq {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut cursor = Cursor::new(Vec::new());
        cursor.push(*self).map_err(|e| serde::ser::Error::custom(e))?;
        serializer.serialize_bytes(cursor.into_inner().as_slice())
    }
}
impl<'de> Deserialize<'de> for Vlq {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
        where
            D: Deserializer<'de> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        let mut cursor = Cursor::new(bytes);
        cursor.pop::<Vlq>().map_err(|e| serde::de::Error::custom(e))
    }
}