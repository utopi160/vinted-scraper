use std::fs::{self, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    basic_search: Vec<configuration::BasicSearch>
}

pub mod configuration {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct BasicSearch {
        path: String,
        vip: bool
    }
}

impl Configuration {
    pub fn get() -> Self {
        let path = "./configuration.json";
        let file = fs::read_to_string(path);

        if let Ok(text) = file {
            return serde_json::from_str::<Configuration>(text.as_str()).unwrap();
        }

        File::create(path).unwrap();
        panic!("Unable to find the file <<{}>>", path);
        
    }
}