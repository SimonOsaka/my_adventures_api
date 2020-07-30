use crate::handlers::{my_date_format, my_item_type_format, my_source};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdventuresResponse {
    pub adventures: Vec<Adventures>,
    pub adventures_count: u64,
}

impl From<Vec<domain::Adventures>> for AdventuresResponse {
    fn from(ads: Vec<domain::Adventures>) -> Self {
        let adventures_count = ads.len() as u64;
        let adventures = ads
            .into_iter()
            .map(|a| Adventures {
                id: a.id,
                title: a.title,
                image_url: a.image_url,
                created_at: a.created_at,
                author_name: "油油".to_string(),
                item_type: a.item_type,
                item_type_name: my_item_type_format::to_item_type_name(a.item_type),
                link: a.link,
                source_name: my_source::to_source_name(a.source),
            })
            .collect();
        Self {
            adventures,
            adventures_count,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdventureResponse {
    pub adventure: Adventures,
}

impl From<domain::Adventures> for AdventureResponse {
    fn from(ad: domain::Adventures) -> Self {
        let adventure = Adventures {
            id: ad.id,
            title: ad.title,
            image_url: ad.image_url,
            created_at: ad.created_at,
            author_name: "油油".to_string(),
            item_type: ad.item_type,
            item_type_name: my_item_type_format::to_item_type_name(ad.item_type),
            link: ad.link,
            source_name: my_source::to_source_name(ad.source),
        };
        Self { adventure }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Adventures {
    pub id: u64,
    pub title: String,
    pub image_url: String,
    #[serde(with = "my_date_format")]
    pub created_at: DateTime<FixedOffset>,
    pub author_name: String,
    pub item_type: u8,
    pub item_type_name: String,
    pub link: String,
    pub source_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InsertResponse {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeletedResponse {
    pub id: u64,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatedResponse {
    pub id: u64,
    pub updated: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response_404 {
    pub message: String,
}
