use crate::queries::adventures;
use crate::DateTimeUtils;
use crate::Repo;
use anyhow::Error as OpaqueError;
use anyhow::Result;
use domain::DatabaseError;

// /// Helper function to cast a diesel::Error into a domain Database Error.
// /// This requires casting the diesel::Error into anyhow::Error first.
pub fn to_db_error(e: OpaqueError) -> domain::DatabaseError {
    domain::DatabaseError::from(e)
}

#[derive(Clone, Debug)]
pub struct Repository(pub Repo);

#[async_trait]
impl domain::repositories::Repository for Repository {
    async fn find_adventures(
        &self,
        query: domain::AdventuresQuery,
    ) -> Result<Vec<domain::Adventures>, DatabaseError> {
        let my_list_result;
        if query.item_id != 0 {
            my_list_result = adventures::find_by_item_type(&self.0, query).await;
        } else {
            my_list_result = adventures::find_latest(&self.0, query).await;
        }
        let result: Vec<domain::Adventures> = my_list_result
            .map_err(to_db_error)
            .unwrap()
            .into_iter()
            .map(|m| domain::Adventures {
                id: m.id,
                title: m.title,
                image_url: m.image_url,
                created_at: DateTimeUtils::beijing(m.created_at),
                item_type: m.item_type,
                link: m.link,
                source: m.source,
                journey_destiny: m.journey_destiny,
                script_content: m.script_content,
                play_list: m.play_list,
            })
            .collect();
        Ok(result)
    }

    async fn find_adventures_by_play_list(
        &self,
        query: domain::PlayListQuery,
    ) -> Result<Vec<domain::Adventures>, DatabaseError> {
        let my_list_result = adventures::find_by_play_list(&self.0, query).await;
        let result: Vec<domain::Adventures> = my_list_result
            .map_err(to_db_error)
            .unwrap()
            .into_iter()
            .map(|m| domain::Adventures {
                id: m.id,
                title: m.title,
                image_url: m.image_url,
                created_at: DateTimeUtils::beijing(m.created_at),
                item_type: m.item_type,
                link: m.link,
                source: m.source,
                journey_destiny: m.journey_destiny,
                script_content: m.script_content,
                play_list: m.play_list,
            })
            .collect();
        Ok(result)
    }

    async fn update_adventure(
        &self,
        update: domain::AdventuresUpdate,
    ) -> Result<bool, DatabaseError> {
        let updated: bool = adventures::update(&self.0, update.into())
            .await
            .map_err(to_db_error)
            .unwrap();

        Ok(updated)
    }

    async fn delete_adventure(&self, _id: u64) -> Result<bool, DatabaseError> {
        let deleted = adventures::delete(&self.0, _id).await?;

        Ok(deleted)
    }
    async fn insert_adventure(
        &self,
        draft: domain::AdventureContent,
    ) -> Result<u64, DatabaseError> {
        let id: u64 = adventures::insert(&self.0, draft.into()).await.unwrap();

        Ok(id)
    }

    async fn get_adventure_by_id(
        &self,
        id: u64,
    ) -> Result<Option<domain::Adventures>, DatabaseError> {
        let my = adventures::find_one(&self.0, id)
            .await
            .map_err(to_db_error)
            .unwrap();

        let result = match my {
            Some(ad) => Some(domain::Adventures {
                id: ad.id,
                title: ad.title,
                image_url: ad.image_url,
                created_at: DateTimeUtils::beijing(ad.created_at),
                item_type: ad.item_type,
                link: ad.link,
                source: ad.source,
                journey_destiny: ad.journey_destiny,
                script_content: ad.script_content,
                play_list: ad.play_list,
            }),
            _ => None,
        };

        Ok(result)
    }
}
