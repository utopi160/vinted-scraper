use std::{thread, time::Duration};

use crate::{models::{config::Configuration, webhook::Webhook, embed::Embed}, constant::BLUE};

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
            
            tokio::time::sleep(duration);
        }
    }
}
