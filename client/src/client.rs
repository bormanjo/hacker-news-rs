use serde::{Deserialize, Serialize, de::DeserializeOwned};

const API_VER: &str = "v0";
const API_URL: &str = "https://hacker-news.firebaseio.com";


#[derive(Deserialize, Serialize)]
pub struct Item {
    id: u32,
    deleted: Option<bool>,
    #[serde(rename = "type")] 
    type_: String,
    by: String,
    time: u32,
    text: Option<String>,
    dead: Option<bool>,
    parent: Option<u32>,
    title: String,
    score: i32,
}

pub struct HNClient {
    client: reqwest::Client,
    url: url::Url,
}


impl HNClient {
    pub fn new() -> Self {
        let url = url::Url::parse(API_URL).expect("Invalid URL");
        let client = reqwest::Client::new();
        HNClient { client, url }
    }

    pub async fn get_item(&self, item_id: u32) -> Result<Item, Box<dyn std::error::Error>> {
        let endpoint = format!("/item/{}.json", item_id.to_string());
        let item = self.get::<Item>(endpoint).await?;
        Ok(item)
    }

    async fn get<T>(&self, endpoint: String) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let endpoint = format!("{}/{}", API_VER, endpoint);
        let url = self.url.join(endpoint.as_str())
            .expect(format!("Invalid endpoint: {}", endpoint).as_str());
        println!("{:#?}", url.to_string());

        let resp = self.client.get(url)
            .send()
            .await?;
        println!("{:#?}", resp);

        let deserialized_item: T = resp.json().await?;
        Ok(deserialized_item)
    }

}