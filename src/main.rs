use std::{thread, time::Duration};

use crate::models::config::Configuration;

mod models;
mod vinted;

fn main() {
    println!("Loading the configuration file ...");
    let config = Configuration::get();

    thread::spawn(move || {
        let duration = Duration::from_secs(10);
        
        loop {
            for search in &config.basic_search  {
                
                tokio::time::sleep(duration);
            }
        }
    });
}
