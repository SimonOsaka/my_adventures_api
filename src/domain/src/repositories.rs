use anyhow::Result;

use crate::{
    AdventuresQuery, AdventuresUpdate, AdventureContent, Adventures, DatabaseError
};

#[async_trait]
pub trait Repository {
    // adventures
    async fn find_adventures(&self, query: AdventuresQuery) -> Result<Vec<Adventures>, DatabaseError>;
    async fn update_adventure(&self, update: AdventuresUpdate) -> Result<bool, DatabaseError>;
    async fn delete_adventure(&self, _id: u64) -> Result<bool, DatabaseError>;
    async fn insert_adventure(&self, draft: AdventureContent) -> Result<u64, DatabaseError>;
    async fn get_adventure_by_id(&self, id: u64) -> Result<Option<Adventures>, DatabaseError>;
}
