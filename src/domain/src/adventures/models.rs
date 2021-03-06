use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct AdventuresQuery {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub province_key: Option<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Adventures {
    pub id: u64,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub item_type: u8,
    pub link: String,
    pub source: u8,
    pub journey_destiny: String,
    pub script_content: String,
    pub play_list: String,
    pub address: String,
    pub shop_name: String,
    pub province: String,
    pub city: String,
    pub district: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdventuresUpdate {
    pub id: u64,
    pub title: String,
    pub image_url: String,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct AdventureContent {
    pub title: String,
    pub image_url: String,
}

#[derive(Clone, Debug)]
pub struct PlayListQuery {
    pub play_list: String,
}
