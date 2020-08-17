use chrono::{DateTime, FixedOffset};
use serde::Serialize;

#[derive(Clone, Debug)]
pub struct AdventuresQuery {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Adventures {
    pub id: u64,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime<FixedOffset>,
    pub item_type: u8,
    pub link: String,
    pub source: u8,
    pub journey_destiny: String,
    pub script_content: String,
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
