use serde::Serialize;
use crate::constant::RED;

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

    pub async fn send_errror(message: String) {
        let mut webhook = Self::new();
        webhook.embeds.insert(0, Embed {
            title: String::from("__**Une erreur est survenue :"),
            description: message,
            color: RED,
            image: None
        });
    }
}