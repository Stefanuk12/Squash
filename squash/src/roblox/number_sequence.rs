use super::prelude::*;

#[derive(From, Into, IntoIterator, AsRef, AsMut, Index, Deref, Mul, IndexMut, DerefMut, MulAssign, Deserialize)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize)]
#[serde(bound(deserialize = "T: SquashNumber"))]
pub struct NumberSequence<T: SquashNumber>(pub Vec<NumberSequenceKeypoint<T>>);
impl_squash!(NumberSequence<T: SquashNumber>);