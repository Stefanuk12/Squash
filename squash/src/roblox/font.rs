use super::prelude::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize)]
pub struct Font {
    pub family: String,
    pub bold: bool,
    pub weight: EnumItem,
    pub style: EnumItem
}
impl_squash!(Font, family, bold, weight, style;style, weight, bold, family);