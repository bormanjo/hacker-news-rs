use itertools::Itertools;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::task::JoinError;

use super::types::{Item, StoryKind};


const API_VER: &str = "v0";
const API_URL: &str = "https://hacker-news.firebaseio.com";

#[derive(Debug)]
pub struct HNClient {
    client: reqwest::Client,
    url: url::Url,
}


impl HNClient {
    pub fn new() -> Arc<Self> {
        let url = url::Url::parse(API_URL).expect("Invalid URL");
        let client = reqwest::Client::new();
        Arc::new(HNClient { client, url })
    }
    
    async fn get<T>(&self, endpoint: String) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        let endpoint = format!("{}/{}", API_VER, endpoint);
        let url = self.url.join(endpoint.as_str())
            .expect(format!("Invalid endpoint: {}", endpoint).as_str());
        log::debug!("{:#?}", url.to_string());
    
        let resp = self.client.get(url)
            .send()
            .await?;
        //log::debug!("{:#?}", resp);
    
        let deserialized_item: T = resp.json().await?;
        Ok(deserialized_item)
    }

    pub async fn get_item(&self, item_id: u32) -> Item {
        let endpoint = format!("/item/{}.json", item_id.to_string());
        self.get::<Item>(endpoint)
            .await
            .expect(format!("Failed to get item: {}", item_id).as_str())
    }

    pub async fn get_stories(&self, kind: &StoryKind) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let val = match kind {
            StoryKind::Best => "beststories",
            StoryKind::New => "newstories",
            StoryKind::Top => "topstories",
        };
        
        let endpoint = format!("/{}.json", val);
        let items = self.get::<Vec<u32>>(endpoint).await?;
        Ok(items)
    }

    pub async fn get_items(self: &Arc<Self>, item_ids: &Vec<u32>)-> Vec<Item> {
        // allocate a vector of task handles
        let mut handles = Vec::with_capacity(item_ids.len());
        for item_id in item_ids {
            // create a threadsafe pointer to this client before move
            // -> enables a spawned task to call client's methods
            let handle = tokio::spawn({
                let client = self.clone();
                let item_id = *item_id;  // copy of id's value moved below
                async move {
                    client.get_item(item_id).await
                }
            });
            handles.push(handle);
        }
    
        // collect results into failures, items
        let results = futures::future::join_all(handles).await;
        let (failures, items): (Vec<JoinError>, Vec<Item>) = results.into_iter().partition_map(Into::into);
        for failed_request in failures {
            log::error!("{}", failed_request.to_string());
        }

        items
    }
}
