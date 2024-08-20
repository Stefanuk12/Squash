use super::prelude::*;

#[derive(From, Into, IntoIterator, AsRef, AsMut, Index, Deref, Mul, IndexMut, DerefMut, MulAssign)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct ColorSequence(pub Vec<ColorSequenceKeypoint>);
impl_squash!(ColorSequence);