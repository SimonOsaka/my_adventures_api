use crate::app::AppState;
use crate::response::Response;
use domain::repositories::Repository;
use serde::{Deserialize, Serialize};

use std::convert::Infallible;
use warp::http::StatusCode;

#[derive(Serialize, Deserialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

pub async fn tags(state: AppState) -> Result<impl warp::Reply, Infallible> {
    // let repository = &state.repository;
    // let tags = repository.get_tags()?;
    // let response = TagsResponse {
    //     tags: tags.into_iter().collect(),
    // };
    // Ok(warp::reply::json(&response))
    Ok(StatusCode::OK)
}
