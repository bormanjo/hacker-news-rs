use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Getters, Debug)]
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
    url: Option<String>,
}


#[derive(Debug, Clone)]
pub enum StoryKind {Top, Best, New}

pub const STORY_KINDS: [StoryKind; 3] = [StoryKind::Top, StoryKind::Best, StoryKind::New];
