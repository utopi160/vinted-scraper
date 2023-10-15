use std::collections::HashMap;
use super::item::Item;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CatalogItems {
    byId: HashMap<u32, Item>
}