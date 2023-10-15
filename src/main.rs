use std::{time::Duration};
use crate::{models::{config::Configuration}};
use crate::vinted::vinted_process_catalog;

mod models;
mod vinted;
mod constant;

#[tokio::main]
async fn main() {
    println!("Loading the configuration file ...");
    let config = Configuration::get();
    
    let duration = Duration::from_secs(10);
    loop {
        for search in &config.basic_search  {
            vinted_process_catalog(search.path.clone()).await;
            tokio::time::sleep(duration).await;
        }
    }
}
