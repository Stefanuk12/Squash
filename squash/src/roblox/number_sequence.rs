use super::prelude::*;

#[derive(From, Into, IntoIterator, AsRef, AsMut, Index, Deref, Mul, IndexMut, DerefMut, MulAssign)]
#[derive(Clone, PartialEq, PartialOrd, Debug, Default, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: SquashNumber"))]
pub struct NumberSequence<T: SquashNumber>(pub Vec<NumberSequenceKeypoint<T>>);