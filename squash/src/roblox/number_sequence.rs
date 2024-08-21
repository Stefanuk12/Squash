use super::prelude::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(deserialize = "T: SquashFloat")))]
#[derive(From, Into, IntoIterator, AsRef, AsMut, Index, Deref, Mul, IndexMut, DerefMut, MulAssign)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct NumberSequence<T: SquashFloat>(pub Vec<NumberSequenceKeypoint<T>>);
impl_squash!(NumberSequence<T: SquashFloat>);