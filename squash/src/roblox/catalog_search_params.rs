use super::prelude::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, ReverseDeserialize, SquashObject)]
pub struct CatalogSearchParams {
    pub include_off_sale: bool,
    pub limit: u8,
    pub min_price: u32,
    pub max_price: u32,
    pub creator_name: String,
    pub search_keyworld: String,
    pub sort_type: EnumItem,
    pub sort_aggregration: EnumItem,
    pub category_filter: EnumItem,
    pub sales_type_filter: EnumItem,
    pub asset_types: Vec<EnumItem>,
}