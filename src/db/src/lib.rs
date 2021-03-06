#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

pub mod connection;
pub mod models;
pub mod queries;
pub mod repository;
pub mod types;

pub use repository::Repository;

pub use connection::Repo;

pub struct DateTimeUtils;

use chrono::{DateTime, FixedOffset, Utc};
impl DateTimeUtils {
    pub fn beijing(utc_time: DateTime<Utc>) -> DateTime<FixedOffset> {
        let tz_offset = FixedOffset::east(8 * 3600);
        utc_time.with_timezone(&tz_offset)
    }
}
