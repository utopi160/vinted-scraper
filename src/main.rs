use std::{time::Duration};
use chrono::{NaiveDateTime, Utc, DateTime};
use crate::{models::{config::Configuration}};
use crate::vinted::vinted_process_catalog;

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

            for (id, item) in items  {
                let diff = now - item.photo.high_resolution.timestamp;

                if diff < last_scan {
                    println!("Je trouve ->>>> [{}] {}s | {}s", id, diff, last_scan);
                }
            }

            config.basic_search[id].last_scan = Some(now);
            tokio::time::sleep(duration).await;
        }
    }
}
