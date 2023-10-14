use super::embed::Embed;

pub struct Webhook {
    url: String,
    embeds: Vec<Embed>
}