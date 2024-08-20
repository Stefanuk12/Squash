use super::prelude::*;

#[derive(From, Into, FromStr, AsRef, AsMut, derive_more::Debug, derive_more::Display, Index, Deref, Not, Add, Mul, Sum, IndexMut, DerefMut, AddAssign, MulAssign)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize)]
pub struct EnumItem(pub Vlq);
impl SquashObject for EnumItem {
    fn pop_obj<T>(cursor: &mut T) -> crate::Result<Self>
            where
                T: crate::SquashCursor,
                Self: Sized {
        Vlq::pop_obj(cursor).map(EnumItem)
    }
    fn push_obj<T: crate::SquashCursor>(self, cursor: &mut T) -> crate::Result<usize> {
        cursor.push(self.0)
    }
}