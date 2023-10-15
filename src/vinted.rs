use std::collections::HashMap;

use crate::models::{webhook::Webhook, vinted::{page::Page, item::Item}, scraper::Scraper};

async fn vinted_get(path: String) -> String {
    let client = reqwest::Client::new();

    let res = client.get(format!("https://www.vinted.fr/{}", path))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/117.0")
        .header("DNT", "1")
        .header("Pragma", "no-cache")
        .header("Cache-Control", "no-cache")
        .header("TE", "trailers")
    .send().await;

    if res.is_err() {
        let error = res.err(); 
        Webhook::send_errror(format!("**Request :** {}\n**Error :** {:?}", path, error)).await;
        panic!("<vinted_get> -> {:?}", error);
    }

    return res.unwrap().text().await.unwrap();
}

pub async fn vinted_process_catalog(search: String) -> HashMap<u32, Item> {
    let page = vinted_get(format!("catalog?{}", search)).await;

    return Scraper::build(r#"{"intl":"#.to_owned(), "{".to_owned())
        .process_json::<Page>(page).items.catalogItems.byId;
}