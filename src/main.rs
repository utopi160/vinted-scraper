use std::time::Duration;
use tokio::time::sleep;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::config::Configuration;
use crate::vinted::vinted_process_catalog;
use crate::models::webhook::Webhook;
use crate::models::embed::Embed;
use crate::models::embed_image::EmbedImage;
use crate::models::ratelimit::Ratelimiter;
use crate::constant::ORANGE;

mod models;
mod vinted;
mod constant;

#[tokio::main]
async fn main() {
    println!("Loading the configuration file ...");
    let config = Configuration::get();
    let ratelimit = Arc::new(Mutex::new(Ratelimiter::new(8, Duration::from_secs(9)))); // May beeee

    let mut threads = Vec::new();

    let items_to_look_lens = config.basic_search.len();
    let items_per_thread = config.basic_search.len().div_ceil(config.threads as usize);
    let mut last_item_id = 0;

    println!("Scraper starting whith {} items ...", config.basic_search.len());

    for thread_id in 0..config.threads {
        let mut idx = last_item_id + items_per_thread;

        if idx > items_to_look_lens {
            idx = items_to_look_lens;
        }

        let mut search_list = config.basic_search[
            last_item_id.. if idx > items_to_look_lens { items_to_look_lens } else { idx }
        ].to_vec();

        last_item_id = idx;

        if search_list.len() <= 0 {
            continue;
        }

        let ratelimit_clone = ratelimit.clone();
        println!("[VINTED] - Creating a Thread ID: {} ({}).", thread_id, search_list.len());

        let handle = tokio::spawn(async move {
            loop {
                let search_list_cloned = search_list.clone();
                for (id, search) in search_list_cloned.iter().enumerate()  {

                    if !ratelimit_clone.lock().await.try_execute().await {
                        sleep(Duration::from_secs(10)).await;
                        continue;
                    }

                    let items = vinted_process_catalog(&search.path).await;

                    if items.len() == 0 { // Rate limit exeded 
                        continue;
                    }

                    let now = Utc::now().timestamp();
                    let last_scan = now - search.last_scan.unwrap_or(now);
                    
                    let webhook_url = search.webhook.clone();
        
                    println!("Fetch (#{}) of {} items", thread_id, items.len());

                    for (_, item) in items  {
                        if item.photo.is_some() {
                            let photo = item.photo.unwrap();
                            let diff = now - photo.high_resolution.timestamp;
        
                            if diff < last_scan {
                                println!("Sends #{} -> {} ({}s)", thread_id, item.id, diff);
            
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

                    search_list[id].last_scan = Some(now);
                }
            }
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.await.unwrap();
    }
}