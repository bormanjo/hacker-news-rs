use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Getters)]
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

pub enum Story {Top, Best, New}
