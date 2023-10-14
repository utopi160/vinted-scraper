async fn vinted_get(path: String) {
    let client = reqwest::Client::new();
    let res = client
        .get(format!("https://www.vinted.fr/{}", path))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .header("DNT", "1")
            .header("Pragma", "no-cache")
            .header("Cache-Control", "no-cache")
            .header("TE", "trailers")
        .send().await;

    if res.is_err() {
        
        panic!("");
    }
    

}