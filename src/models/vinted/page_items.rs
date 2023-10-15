use serde::Deserialize;
use super::catalog_items::CatalogItems;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]

pub struct PageItems {
    catalogItems: CatalogItems
}