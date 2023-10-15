use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    id: u32,
    title: String
}