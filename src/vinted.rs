use std::fmt::format;

use crate::models::webhook::Webhook;

async fn vinted_get(path: String) {
    let res = reqwest::Client::new()
        .get(format!("https://www.vinted.fr/{}", path))
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
}