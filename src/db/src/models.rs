// use crate::schema::articles;
// use crate::schema::comments;
// use crate::schema::favorites;
// use crate::schema::followers;
// use crate::schema::users;
use chrono::{DateTime, Utc};
// use diesel::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;

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
// #[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
// pub struct User {
//     pub id: Uuid,
//     pub username: String,
//     pub email: String,
//     pub password: String,
//     pub bio: Option<String>,
//     pub image: Option<String>,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// #[derive(Serialize, Deserialize, Debug, AsChangeset, Default, Clone)]
// #[table_name = "users"]
// pub struct UpdateUser<'a> {
//     pub email: Option<&'a str>,
//     pub username: Option<&'a str>,
//     pub password: Option<String>,
//     pub image: Option<&'a str>,
//     pub bio: Option<&'a str>,
// }

// #[derive(Queryable, Serialize, Deserialize, Debug, PartialEq)]
// pub struct Article {
//     pub title: String,
//     pub slug: String,
//     pub description: String,
//     pub body: String,
//     pub tag_list: Vec<String>,
//     pub user_id: Uuid,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "users"]
// pub struct NewUser<'a> {
//     pub id: Uuid,
//     pub username: &'a str,
//     pub email: &'a str,
//     pub password: &'a str,
// }

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "articles"]
// pub struct NewArticle<'a> {
//     pub title: &'a str,
//     pub slug: String,
//     pub description: &'a str,
//     pub body: &'a str,
//     pub tag_list: Vec<String>,
//     pub user_id: Uuid,
// }

// #[derive(AsChangeset, Deserialize, Debug, Clone)]
// #[table_name = "articles"]
// pub struct UpdateArticle<'a> {
//     pub title: Option<&'a str>,
//     pub description: Option<&'a str>,
//     pub body: Option<&'a str>,
// }

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "favorites"]
// pub struct NewFavorite {
//     pub user_id: Uuid,
//     pub article_id: String,
// }

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "followers"]
// pub struct NewFollower {
//     pub followed_id: Uuid,
//     pub follower_id: Uuid,
// }

// #[derive(Insertable, Deserialize, Debug, Clone)]
// #[table_name = "comments"]
// pub struct NewComment<'a> {
//     pub author_id: Uuid,
//     pub article_id: &'a str,
//     pub body: &'a str,
// }

// #[derive(Queryable, Deserialize, Debug, Clone)]
// pub struct Comment {
//     pub id: i64,
//     pub author_id: Uuid,
//     pub article_id: String,
//     pub body: String,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
// }
