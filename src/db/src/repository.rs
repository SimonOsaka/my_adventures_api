use crate::queries::adventures;
use crate::types::{convert_i16_u8, convert_id, convert_id_u64, convert_ts, SqlID};
use crate::Repo;
use anyhow::Error as OpaqueError;
use anyhow::Result;
use domain::DatabaseError;
use sqlx::Error;

// /// Helper function to cast a diesel::Error into a domain Database Error.
// /// This requires casting the diesel::Error into anyhow::Error first.
pub fn to_db_error(e: Error) -> DatabaseError {
    let oe = OpaqueError::from(e);
    DatabaseError::from(oe)
}

pub fn to_db_error1(e: anyhow::Error) -> DatabaseError {
    DatabaseError::from(e)
}

#[derive(Clone, Debug)]
pub struct Repository(pub Repo);

#[async_trait]
impl domain::repositories::Repository for Repository {
    async fn find_adventures(
        &self,
        query: domain::AdventuresQuery,
    ) -> Result<Vec<domain::Adventures>, DatabaseError> {
        let my_list_result = adventures::find_latest(&self.0, query).await;
        let result: Vec<domain::Adventures> = my_list_result
            .map_err(to_db_error1)
            .unwrap()
            .into_iter()
            .map(|m| domain::Adventures {
                id: convert_id(m.id),
                title: m.title,
                image_url: m.image_url,
                created_at: convert_ts(m.created_at),
                item_type: convert_i16_u8(m.item_type),
                link: m.link,
                source: convert_i16_u8(m.source),
                journey_destiny: m.journey_destiny,
                script_content: m.script_content,
                play_list: m.play_list,
                address: m.address,
                shop_name: m.shop_name,
                province: m.province,
                city: m.city,
                district: m.district,
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
            .map_err(to_db_error1)
            .unwrap()
            .into_iter()
            .map(|m| domain::Adventures {
                id: convert_id(m.id),
                title: m.title,
                image_url: m.image_url,
                created_at: convert_ts(m.created_at),
                item_type: convert_i16_u8(m.item_type),
                link: m.link,
                source: convert_i16_u8(m.source),
                journey_destiny: m.journey_destiny,
                script_content: m.script_content,
                play_list: m.play_list,
                address: m.address,
                shop_name: m.shop_name,
                province: m.province,
                city: m.city,
                district: m.district,
            })
            .collect();
        Ok(result)
    }

    async fn update_adventure(
        &self,
        update: domain::AdventuresUpdate,
    ) -> Result<bool, DatabaseError> {
        let updated: bool =
            adventures::update(&self.0, update.into()).await.map_err(to_db_error).unwrap();

        Ok(updated)
    }

    async fn delete_adventure(&self, _id: u64) -> Result<bool, DatabaseError> {
        let deleted =
            adventures::delete(&self.0, convert_id_u64(_id)).await.map_err(to_db_error).unwrap();

        Ok(deleted)
    }
    async fn insert_adventure(
        &self,
        draft: domain::AdventureContent,
    ) -> Result<u64, DatabaseError> {
        let id: SqlID = adventures::insert(&self.0, draft.into()).await.unwrap();

        Ok(convert_id(id))
    }

    async fn get_adventure_by_id(
        &self,
        id: u64,
    ) -> Result<Option<domain::Adventures>, DatabaseError> {
        let my = adventures::find_one(&self.0, id).await.map_err(to_db_error1).unwrap();

        let result = match my {
            Some(ad) => Some(domain::Adventures {
                id: convert_id(ad.id),
                title: ad.title,
                image_url: ad.image_url,
                created_at: convert_ts(ad.created_at),
                item_type: convert_i16_u8(ad.item_type),
                link: ad.link,
                source: convert_i16_u8(ad.source),
                journey_destiny: ad.journey_destiny,
                script_content: ad.script_content,
                play_list: ad.play_list,
                address: ad.address,
                shop_name: ad.shop_name,
                province: ad.province,
                city: ad.city,
                district: ad.district,
            }),
            _ => None,
        };

        Ok(result)
    }
}
