use std::collections::HashMap;
use super::item::Item;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CatalogItems {
    pub byId: HashMap<u32, Item>
}