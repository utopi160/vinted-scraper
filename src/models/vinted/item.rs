use serde::Deserialize;
use super::{item_photo::ItemPhoto, item_price::ItemPrice};

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: u32, /* Identifier */
    pub title: String, /* Title */
    pub url: String, /* url */

    pub brand_title: String, /* Mark */
    pub size_title: String, /* Size of item */

    pub total_item_price: ItemPrice, /* Price */
    pub photo: Option<ItemPhoto> /* Photo and timestamp */
}