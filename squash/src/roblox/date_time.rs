use crate::u48;

use super::prelude::*;

#[derive(From, Into, AsRef, AsMut, derive_more::Debug, derive_more::Display, Index, Deref, Mul, IndexMut, DerefMut, MulAssign)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Serialize, Deserialize)]
pub struct DateTime(pub u48);
impl_squash!(DateTime);