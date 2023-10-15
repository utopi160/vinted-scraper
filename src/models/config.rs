use std::fs::{self, File};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Configuration {
    pub basic_search: Vec<configuration::BasicSearch>
}

pub mod configuration {
    use serde::Deserialize;

    #[derive(Deserialize, Clone, Debug)]
    pub struct BasicSearch {
        pub path: String,
        pub last_scan: Option<i64>,
        pub vip: bool
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