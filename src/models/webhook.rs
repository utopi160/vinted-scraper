use reqwest::IntoUrl;
use serde::Serialize;
use crate::constant::{RED, WEBHOOK_ERRORS};

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

    pub async fn send<T>(&self, url: T)
    where
        T: IntoUrl
    {
        if let Err(e) = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(self).unwrap()
            )
        .send().await {
            println!("Error discord: -> {}", e);
        };
    }

    pub async fn send_errror(message: String) {
        let mut webhook = Self::new();
        webhook.embeds.insert(0, Embed {
            title: String::from("__**Une erreur est survenue :"),
            description: message,
            color: RED,
            image: None
        });

        webhook.send(WEBHOOK_ERRORS).await;
    }
}