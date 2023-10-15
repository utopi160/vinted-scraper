use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ItemPrice {
    pub amount: String
}