use serde::Deserialize;
use super::{item_photo::ItemPhoto, item_price::ItemPrice};

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub total_item_price: ItemPrice,
    pub photo: Option<ItemPhoto>
}