use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct Items {
    catalogItems: String
}