use crate::models::config::Configuration;

mod models;

fn main() {
    println!("Loading the configuration file ...");
    let config = Configuration::get();

    println!("COnfig file -> {:?}", config);
}
