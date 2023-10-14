use std::{thread, time::Duration};

use crate::{models::{config::Configuration, webhook::Webhook, embed::Embed}, constant::BLUE};

mod models;
mod vinted;
mod constant;

#[tokio::main]
async fn main() {
    println!("Loading the configuration file ...");
    let config = Configuration::get();

    let mut webhook = Webhook::new();
    webhook.embeds.insert(0, Embed {
        title: String::from("Test 1"),
        description: String::from("Je suis énormeeeee"),
        color: BLUE,
        image: None
    });

    webhook.send("https://discord.com/api/webhooks/1158089945295114321/VPOd9Pc04JCIK1o2fNkvqv1LoJEIFBPO0UBO7SDWmAi8GYHSBssTPZ6mZRMaNPuCAfBz".to_owned()).await;
    
    let duration = Duration::from_secs(10);
    loop {
        for search in &config.basic_search  {
            
            tokio::time::sleep(duration);
        }
    }
}
