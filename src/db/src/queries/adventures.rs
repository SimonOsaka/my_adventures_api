use crate::models::{EntityId, MyAdventures, NewMyAdventures, UpdateMyAdventures};
use crate::Repo;
use anyhow::{Error, Result};
use domain;
use domain::{AdventuresQuery, PlayListQuery};
use sqlx::mysql::MySqlDone;
use sqlx::Done;

pub async fn insert(repo: &Repo, adventures_new: NewMyAdventures) -> Result<u64, Error> {
    let mut transaction = (&repo.connection_pool).begin().await?;
    sqlx::query!(
        "INSERT INTO my_adventures( title, image_url ) VALUES( ?, ? )",
        adventures_new.title,
        adventures_new.image_url
    )
    .execute(&mut transaction)
    .await?;

    let entity_id = sqlx::query_as!(EntityId, "SELECT LAST_INSERT_ID() as id")
        .fetch_one(&mut transaction)
        .await?;

    debug!("new adventures id: {}", entity_id.id);
    transaction.commit().await?;

    Ok(entity_id.id)
}

pub async fn update(repo: &Repo, adventures_update: UpdateMyAdventures) -> Result<bool, Error> {
    let d: MySqlDone = sqlx::query!(
        "UPDATE my_adventures SET title = ?, image_url = ? WHERE id = ?",
        adventures_update.title,
        adventures_update.image_url,
        adventures_update.id
    )
    .execute(&repo.connection_pool)
    .await?;

    Ok(d.rows_affected() > 0)
}

pub async fn delete(repo: &Repo, _id: u64) -> Result<bool, Error> {
    let d: MySqlDone = sqlx::query!(
        "UPDATE my_adventures SET is_deleted=1 WHERE is_deleted = 0 and id = ?",
        _id
    )
    .execute(&repo.connection_pool)
    .await?;

    Ok(d.rows_affected() > 0)
}

pub async fn find_latest(
    repo: &Repo,
    query: AdventuresQuery,
) -> anyhow::Result<Vec<MyAdventures>, anyhow::Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        "SELECT id,title,created_at,is_deleted,image_url,item_type,link,source,journey_destiny,script_content,play_list,address FROM my_adventures WHERE is_deleted = 0 ORDER BY id DESC LIMIT ?, ?",
        query.offset,
        query.limit
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

pub async fn find_by_item_type(
    repo: &Repo,
    query: AdventuresQuery,
) -> anyhow::Result<Vec<MyAdventures>, anyhow::Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        "SELECT id,title,created_at,is_deleted,image_url,item_type,link,source,journey_destiny,script_content,play_list,address FROM my_adventures WHERE is_deleted = 0 AND item_type = ? ORDER BY id DESC LIMIT ?, ?",
        query.item_id,
        query.offset,
        query.limit
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

pub async fn find_by_play_list(
    repo: &Repo,
    query: PlayListQuery,
) -> anyhow::Result<Vec<MyAdventures>, anyhow::Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        "SELECT id,title,created_at,is_deleted,image_url,item_type,link,source,journey_destiny,script_content,play_list,address FROM my_adventures WHERE is_deleted = 0 AND play_list = ?",
        query.play_list
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

pub async fn find_one(repo: &Repo, id: u64) -> anyhow::Result<Option<MyAdventures>, anyhow::Error> {
    let my = sqlx::query_as!(
        MyAdventures,
        "SELECT id,title,created_at,is_deleted,image_url,item_type,link,source,journey_destiny,script_content,play_list,address FROM my_adventures WHERE id = ? and is_deleted = 0",
        id
    )
    .fetch_optional(&repo.connection_pool)
    .await?;

    Ok(my)
}
