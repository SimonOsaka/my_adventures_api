use crate::models::{MyAdventures, NewMyAdventures, UpdateMyAdventures};
use crate::Repo;
use anyhow::Result;
use domain;
use domain::{AdventuresQuery, PlayListQuery};
use sqlx::Error;

#[cfg(any(feature = "mysql"))]
pub type SqlDone = sqlx::mysql::MySqlDone;

#[cfg(any(feature = "postgres"))]
pub type SqlDone = sqlx::postgres::PgQueryResult;

#[cfg(any(feature = "mysql"))]
pub async fn insert(repo: &Repo, adventures_new: NewMyAdventures) -> Result<u64, Error> {
    let mut transaction = (&repo.connection_pool).begin().await?;
    sqlx::query!(
        "INSERT INTO my_adventures( title, image_url ) VALUES( ?, ? )",
        adventures_new.title,
        adventures_new.image_url
    )
    .execute(&mut transaction)
    .await?;

    let entity_id = sqlx::query_as!(crate::models::EntityId, "SELECT LAST_INSERT_ID() as id")
        .fetch_one(&mut transaction)
        .await?;

    debug!("new adventures id: {}", entity_id.id);
    transaction.commit().await?;

    Ok(entity_id.id)
}

#[cfg(any(feature = "postgres"))]
pub async fn insert(repo: &Repo, adventures_new: NewMyAdventures) -> Result<i64, Error> {
    let rec = sqlx::query!(
        r#"
        INSERT INTO my_adventures( title, image_url )
        VALUES( $1, $2 )
        RETURNING id
        "#,
        adventures_new.title,
        adventures_new.image_url
    )
    .fetch_one(&repo.connection_pool)
    .await?;

    debug!("new adventures id: {:?}", rec.id);

    Ok(rec.id)
}

#[cfg(any(feature = "mysql"))]
pub async fn update(repo: &Repo, adventures_update: UpdateMyAdventures) -> Result<bool, Error> {
    let d: SqlDone = sqlx::query!(
        "UPDATE my_adventures SET title = ?, image_url = ? WHERE id = ?",
        adventures_update.title,
        adventures_update.image_url,
        adventures_update.id
    )
    .execute(&repo.connection_pool)
    .await?;

    Ok(d.rows_affected() > 0)
}

#[cfg(any(feature = "postgres"))]
pub async fn update(repo: &Repo, adventures_update: UpdateMyAdventures) -> Result<bool, Error> {
    let d: SqlDone = sqlx::query!(
        "UPDATE my_adventures SET title = $1, image_url = $2 WHERE id = $3",
        adventures_update.title,
        adventures_update.image_url,
        adventures_update.id
    )
    .execute(&repo.connection_pool)
    .await?;

    Ok(d.rows_affected() > 0)
}

#[cfg(any(feature = "mysql"))]
pub async fn delete(repo: &Repo, _id: u64) -> Result<bool, Error> {
    let d: SqlDone =
        sqlx::query!("UPDATE my_adventures SET is_deleted=1 WHERE is_deleted = 0 and id = ?", _id)
            .execute(&repo.connection_pool)
            .await?;

    Ok(d.rows_affected() > 0)
}

#[cfg(any(feature = "postgres"))]
pub async fn delete(repo: &Repo, _id: i64) -> Result<bool, Error> {
    let d: SqlDone =
        sqlx::query!("UPDATE my_adventures SET is_deleted=1 WHERE is_deleted = 0 and id = $1", _id)
            .execute(&repo.connection_pool)
            .await?;

    Ok(d.rows_affected() > 0)
}

#[cfg(any(feature = "mysql"))]
pub async fn find_latest(repo: &Repo, query: AdventuresQuery) -> Result<Vec<MyAdventures>, Error> {
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

#[cfg(any(feature = "postgres"))]
pub async fn find_latest(repo: &Repo, query: AdventuresQuery) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,
            source,journey_destiny,script_content,play_list,address,
            shop_name,province,city,district
        FROM
            my_adventures
        WHERE
            is_deleted = 0
        ORDER BY id DESC
        LIMIT $1 OFFSET $2
        "#,
        query.limit.unwrap() as i64,
        query.offset.unwrap() as i64
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "mysql"))]
pub async fn find_by_item_type(
    repo: &Repo,
    query: AdventuresQuery,
) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,source,
            journey_destiny,script_content,play_list,address
        FROM
            my_adventures
        WHERE
            is_deleted = 0 AND item_type = ? ORDER BY id DESC LIMIT ?, ?
        "#,
        query.item_id,
        query.offset,
        query.limit
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "postgres"))]
pub async fn find_by_item_type(
    repo: &Repo,
    query: AdventuresQuery,
) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,
            source,journey_destiny,script_content,play_list,address,
            shop_name,province,city,district
        FROM
            my_adventures
        WHERE
            is_deleted = 0 AND item_type = $1
        ORDER BY id DESC
        LIMIT $2 OFFSET $3
        "#,
        query.item_id as i16,
        query.limit.unwrap() as i64,
        query.offset.unwrap() as i64
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "mysql"))]
pub async fn find_by_play_list(
    repo: &Repo,
    query: PlayListQuery,
) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        "SELECT id,title,created_at,is_deleted,image_url,item_type,link,source,journey_destiny,script_content,play_list,address FROM my_adventures WHERE is_deleted = 0 AND play_list = ?",
        query.play_list
    )
        .fetch_all(&repo.connection_pool)
        .await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "postgres"))]
pub async fn find_by_play_list(
    repo: &Repo,
    query: PlayListQuery,
) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,
            source,journey_destiny,script_content,play_list,address,
            shop_name,province,city,district
        FROM
            my_adventures
        WHERE
            is_deleted = 0 AND play_list = $1
        "#,
        query.play_list
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "mysql"))]
pub async fn find_one(repo: &Repo, id: u64) -> Result<Option<MyAdventures>, Error> {
    let my = sqlx::query_as!(
        MyAdventures,
        "SELECT id,title,created_at,is_deleted,image_url,item_type,link,source,journey_destiny,script_content,play_list,address FROM my_adventures WHERE id = ? and is_deleted = 0",
        id
    )
        .fetch_optional(&repo.connection_pool)
        .await?;

    Ok(my)
}

#[cfg(any(feature = "postgres"))]
pub async fn find_one(repo: &Repo, id: u64) -> Result<Option<MyAdventures>, Error> {
    let my = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,
            source,journey_destiny,script_content,play_list,address,
            shop_name,province,city,district
        FROM
            my_adventures
        WHERE id = $1 and is_deleted = 0
        "#,
        id as i64
    )
    .fetch_optional(&repo.connection_pool)
    .await?;

    Ok(my)
}

#[cfg(any(feature = "mysql"))]
pub async fn find_by_item_type_province(
    repo: &Repo,
    query: AdventuresQuery,
) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,source,
            journey_destiny,script_content,play_list,address
        FROM
            my_adventures
        WHERE
            is_deleted = 0 AND item_type = ? and journey_destiny = ? ORDER BY id DESC LIMIT ?, ?
        "#,
        query.item_id,
        query.province_key,
        query.offset,
        query.limit
    )
    .fetch_all(&repo.connection_pool)
    .await?;

    Ok(my_adventures)
}

#[cfg(any(feature = "postgres"))]
pub async fn find_by_item_type_province(
    repo: &Repo,
    query: AdventuresQuery,
) -> Result<Vec<MyAdventures>, Error> {
    let my_adventures = sqlx::query_as!(
        MyAdventures,
        r#"
        SELECT
            id,title,created_at,is_deleted,image_url,item_type,link,
            source,journey_destiny,script_content,play_list,address,
            shop_name,province,city,district
        FROM
            my_adventures
        WHERE
            is_deleted = 0 AND item_type = $1 and journey_destiny = $2 ORDER BY id DESC LIMIT $3 OFFSET $4
        "#,
        query.item_id as i16,
        query.province_key,
        query.limit.unwrap() as i64,
        query.offset.unwrap() as i64
    )
        .fetch_all(&repo.connection_pool)
        .await?;

    Ok(my_adventures)
}
