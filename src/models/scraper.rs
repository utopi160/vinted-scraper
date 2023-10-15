use serde::de::DeserializeOwned;
use super::vinted::catalog_items::CatalogItems;

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

    pub fn process_json<T>(&self, text: String) -> T 
    where
        T: DeserializeOwned
    {
        let start_idx = text.find(&self.start);

        if start_idx.is_none() {
            panic!("Ahooooooooooooooo");
        }

        let start_of_text = &text[start_idx.unwrap()..];
        let rest_of_json = &start_of_text[start_of_text.find("{").unwrap()..];
        let json_end = rest_of_json.find("</script>").unwrap();

        let json = serde_json::from_str::<T>(&rest_of_json[..json_end]).unwrap();
        return json;
    }
}