use serde::Serialize;
use super::embed::Embed;

#[derive(Serialize)]
pub struct Webhook {
    username: String,
    pub embeds: Vec<Embed>
}

impl Webhook {
    pub fn new() -> Webhook {
        return Webhook {
            username: "L'oeil d'Utopi".to_owned(),
            embeds: Vec::new()
        };
    }

    pub async fn send(&self, url: String) {
        reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(self).unwrap()
            )
        .send().await.unwrap();
    }
}