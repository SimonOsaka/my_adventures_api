use crate::models::{MyAdventures, NewMyAdventures, UpdateMyAdventures};
use crate::queries::SqlParam;
use crate::Repo;
use anyhow::Result;
use domain;
use domain::{AdventuresQuery, PlayListQuery};
use sql_builder::SqlBuilder;

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

#[cfg(any(feature = "postgres", feature = "mysql"))]
pub async fn find_latest(
    repo: &Repo,
    query: AdventuresQuery,
) -> Result<Vec<MyAdventures>, anyhow::Error> {
    let mut pgsql_builder = SqlBuilder::select_from("my_adventures");
    pgsql_builder
        .fields(&[
            "id",
            "title",
            "created_at",
            "is_deleted",
            "image_url",
            "item_type",
            "link",
            "source",
            "journey_destiny",
            "script_content",
            "play_list",
            "address",
            "shop_name",
            "province",
            "city",
            "district",
        ])
        .and_where_eq("is_deleted", 0);

    let mut param = SqlParam::new();

    if query.item_id != 0 {
        match query.province_key.as_ref() {
            // 字符串变量存在
            Some(pv) => {
                if pv.len() > 0 {
                    pgsql_builder
                        .and_where_eq("item_type", &param.value(query.item_id as i16))
                        .and_where_eq("journey_destiny", &param.value(query.province_key.unwrap()));
                } else {
                    pgsql_builder.and_where_eq("item_type", &param.value(query.item_id as i16));
                }
            }
            _ => {
                pgsql_builder.and_where_eq("item_type", &param.value(query.item_id as i16));
            }
        }
    }

    let sql = pgsql_builder
        .order_desc("id")
        .limit(&param.value(query.limit.unwrap() as i64))
        .offset(&param.value(query.offset.unwrap() as i64))
        .sql()?;
    let my_adventures =
        sqlx::query_as_with(&sql, param.fetch_args()).fetch_all(&repo.connection_pool).await?;

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
