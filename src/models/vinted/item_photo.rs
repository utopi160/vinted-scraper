use serde::Deserialize;
use super::item_high_resolution::ItemPhotoHightResolution;

#[derive(Deserialize, Debug)]
pub struct ItemPhoto {
    pub url: String,
    pub high_resolution: ItemPhotoHightResolution
}