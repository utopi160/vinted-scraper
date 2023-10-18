use serde::de::DeserializeOwned;
use super::webhook::Webhook;

pub struct Scraper {
    start: String,
    end: String
}

impl Scraper {
    pub fn build(start: String, end: String) -> Self {
        return Scraper {
            start,
            end
        };
    }

    pub async fn process_json<T>(&self, text: String) -> Option<T>
    where
        T: DeserializeOwned
    {
        let start_idx = text.find(&self.start);

        if start_idx.is_none() {
            println!("Error<process_json>: {:?}", text);
            return None;
        }

        let start_of_text = &text[start_idx.unwrap()..];
        let rest_of_json = &start_of_text[start_of_text.find(&self.end).unwrap()..];
        let json_end = rest_of_json.find("</script>").unwrap();

        let json = serde_json::from_str::<T>(&rest_of_json[..json_end]);

        if let Err(e) = json {
            Webhook::send_errror(format!("Process json -> ** {:?}", e)).await;
            return None
        }

        return Some(json.unwrap());
    }
}