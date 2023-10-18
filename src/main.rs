use std::{time::Duration};
use chrono::Utc;
use crate::{models::{config::Configuration}};
use crate::vinted::vinted_process_catalog;
use crate::models::webhook::Webhook;
use crate::models::embed::Embed;
use crate::models::embed_image::EmbedImage;
use crate::constant::{ORANGE};

mod models;
mod vinted;
mod constant;

#[tokio::main]
async fn main() {
    println!("Loading the configuration file ...");
    let mut config = Configuration::get();
    
    let duration = Duration::from_secs(10);
    loop {
        for (id, search) in config.basic_search.clone().iter().enumerate() {
            let items = vinted_process_catalog(search.path.clone()).await;
            let now = Utc::now().timestamp();
            let last_scan = now - search.last_scan.unwrap_or(now);

            for (_, item) in items  {
                if item.photo.is_some() {
                    let photo = item.photo.unwrap();
                    let diff = now - photo.high_resolution.timestamp;

                    if diff < last_scan {
                        println!("J'envoie -> {} ({}s)", item.id, diff);
    
                        let mut webhook = Webhook::new();
                        webhook.embeds.insert(0, Embed { 
                            title: String::from("__**Nouveau Article :**__"), 
                            description: format!("**ID :** #{}\n**Titre :** {}\n**Prix :** {}€\n\n{}\n\nIl y a <t:{}:R> ", item.id, item.title, item.total_item_price.amount, item.url, photo.high_resolution.timestamp),
                            image: Some(EmbedImage {
                                url: photo.url
                            }), color: ORANGE 
                        });
    
                        webhook.send("https://discord.com/api/webhooks/1163096654879137874/2Jy6yuHow-Nbnr8neP1p1MviqcyA-ufoaBLZmJTRTEn2gCixT4p9faq3jBR0NW_H1FWC").await;
                    }
                }
            }

            config.basic_search[id].last_scan = Some(now);
            tokio::time::sleep(duration).await;
        }
    }
}
