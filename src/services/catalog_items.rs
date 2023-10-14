use std::collections::HashMap;
use super::item::Item;

#[allow(non_snake_case)]
pub struct CatalogItems {
    byId: HashMap<u32, Item>
}