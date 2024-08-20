use super::prelude::*;

#[derive(From, Into, FromStr, AsRef, AsMut, derive_more::Debug, derive_more::Display, Index, Deref, Not, Add, Mul, Sum, IndexMut, DerefMut, AddAssign, MulAssign)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize)]
pub struct BrickColor(pub u16);
impl_squash!(BrickColor);

#[cfg(test)]
mod tests {
    use crate::{deserialize, serialize};

    use super::*;

    #[test]
    fn brick_color() {
        let value = 69u16;
        let x = BrickColor(value);
        let mut ser = serialize(&x).unwrap();
        assert_eq!(ser, value.to_le_bytes());
        let des = deserialize(&mut ser).unwrap();
        assert_eq!(x, des);
    }
}