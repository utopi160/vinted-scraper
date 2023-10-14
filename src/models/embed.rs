use serde::Serialize;
use super::embed_image::EmbedImage;

#[derive(Serialize)]
pub struct Embed {
    pub title: String, 
    pub description: String,
    pub image: Option<EmbedImage>,
    pub color: u32
}