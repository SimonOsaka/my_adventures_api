use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EntityId {
    pub id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct MyAdventures {
    pub id: u64,
    pub title: String,
    pub image_url: String,
    pub created_at: DateTime<Utc>,
    pub is_deleted: u8,
    pub item_type: u8,
    pub link: String,
    pub source: u8,
    pub journey_destiny: String,
    pub script_content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewMyAdventures {
    pub title: String,
    pub image_url: String,
}

impl From<domain::AdventureContent> for NewMyAdventures {
    fn from(content: domain::AdventureContent) -> Self {
        Self {
            title: content.title,
            image_url: content.image_url,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateMyAdventures {
    pub id: u64,
    pub title: String,
    pub image_url: String,
}

impl From<domain::AdventuresUpdate> for UpdateMyAdventures {
    fn from(update: domain::AdventuresUpdate) -> Self {
        Self {
            id: update.id,
            title: update.title,
            image_url: update.image_url,
        }
    }
}
