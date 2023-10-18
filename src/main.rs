use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    let config = Configuration::get();
    
    let duration = Duration::from_secs(10);
    let mut last_scans = Arc::new(Mutex::new(HashMap::new()));
    let mut queries = 0;

    // config.basic_search[id_cloned].last_scan = Some(now);

    loop {
        for (id, search) in config.basic_search.clone().iter().enumerate() {
            let cloned_search = search.clone();
            let cloned_id = id.clone();

            let last_scans_clone = Arc::clone(&last_scans);

            tokio::spawn(async move {
                queries += 1;
            
                if queries > 28 {
                    tokio::time::sleep(duration).await;
                }
    
                let items = vinted_process_catalog(cloned_search.path.clone()).await;
                let now = Utc::now().timestamp();
                let last_scan = now - cloned_search.last_scan.unwrap_or(now);
                
                let webhook_url = cloned_search.webhook.clone();
    
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
        
                            webhook.send(&webhook_url).await;
                        }
                    }
                }

                let mut last_scans_guard = last_scans_clone.lock().unwrap();
                last_scans_guard.insert(cloned_id, now);
            });
        }
    }
}