use super::prelude::*;

#[derive(From, Into, FromStr, AsRef, AsMut, derive_more::Debug, derive_more::Display, Index, Deref, Not, Add, Mul, Sum, IndexMut, DerefMut, AddAssign, MulAssign)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize)]
pub struct EnumItem(pub Vlq);
impl_squash!(EnumItem);