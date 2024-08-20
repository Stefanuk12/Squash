use super::prelude::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct Font {
    pub family: String,
    pub bold: bool,
    pub weight: EnumItem,
    pub style: EnumItem
}