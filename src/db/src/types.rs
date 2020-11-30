// SqlPool
#[cfg(any(feature = "mysql"))]
pub type SqlPool = sqlx::MySqlPool;
#[cfg(any(feature = "postgres"))]
pub type SqlPool = sqlx::PgPool;

// PoolOptions
#[cfg(any(feature = "mysql"))]
pub type PoolOptions = sqlx::mysql::MySqlPoolOptions;
#[cfg(any(feature = "postgres"))]
pub type PoolOptions = sqlx::postgres::PgPoolOptions;

// SqlID
use chrono::{DateTime, Utc};
#[cfg(any(feature = "mysql"))]
pub type SqlID = u64;
#[cfg(any(feature = "postgres"))]
pub type SqlID = i64;

// SqlDateTime
#[cfg(any(feature = "mysql"))]
pub type SqlDateTime = chrono::DateTime<Utc>;
#[cfg(any(feature = "postgres"))]
pub type SqlDateTime = chrono::NaiveDateTime;

// SqlIsDeleted
#[cfg(any(feature = "mysql"))]
pub type SqlU8I16 = u8;
#[cfg(any(feature = "postgres"))]
pub type SqlU8I16 = i16;

#[cfg(any(feature = "postgres"))]
pub fn convert_id(id: i64) -> u64 {
    id as u64
}

#[cfg(any(feature = "mysql"))]
pub fn convert_id(id: u64) -> u64 {
    id
}

#[cfg(any(feature = "postgres"))]
pub fn convert_id_u64(id: u64) -> i64 {
    id as i64
}

#[cfg(any(feature = "mysql"))]
pub fn convert_id_u64(id: u64) -> u64 {
    id
}

#[cfg(any(feature = "postgres"))]
pub fn convert_i16_u8(i: i16) -> u8 {
    i as u8
}

#[cfg(any(feature = "mysql"))]
pub fn convert_i16_u8(i: u8) -> u8 {
    i
}

#[cfg(any(feature = "postgres"))]
pub fn convert_ts(ts: chrono::NaiveDateTime) -> DateTime<Utc> {
    chrono::DateTime::from_utc(ts, chrono::Utc)
}

#[cfg(any(feature = "mysql"))]
pub fn convert_ts(ts: DateTime<Utc>) -> DateTime<Utc> {
    ts
}
