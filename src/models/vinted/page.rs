use serde::Deserialize;
use super::page_items::PageItems;

#[derive(Deserialize, Debug)]
pub struct Page {
    pub items: PageItems
}