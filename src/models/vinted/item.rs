use serde::Deserialize;
use super::item_photo::ItemPhoto;

#[derive(Deserialize, Debug)]
pub struct Item {
    pub id: u32,
    pub title: String,
    pub photo: ItemPhoto
}