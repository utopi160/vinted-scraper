use std::time::Duration;
use chrono::Utc;
use crate::models::config::Configuration;
use crate::vinted::vinted_process_catalog;
use crate::models::webhook::Webhook;
use crate::models::embed::Embed;
use crate::models::embed_image::EmbedImage;
use crate::constant::ORANGE;

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
                            description: format!(
                                "**Titre :** {}\n**Marque :** {}\n**Taille :** {}\n**Prix :** {}€\n**Posté :**  <t:{}:R>\n\n{}",
                                item.title, item.brand_title, item.size_title, item.total_item_price.amount, photo.high_resolution.timestamp,
                                item.url
                            ),

                            image: Some(EmbedImage {
                                url: photo.url
                            }), color: ORANGE 
                        });
    
                        webhook.send(&search.webhook).await;
                    }
                }
            }

            config.basic_search[id].last_scan = Some(now);
            tokio::time::sleep(duration).await;
        }
    }
}
