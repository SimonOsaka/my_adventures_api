use crate::app::AppState;
use crate::handlers::adventures::responses::AdventuresResponse;
use domain::repositories::Repository;
use serde::Deserialize;
use std::convert::Infallible;

// use crate::auth::decode_token;
// use crate::response::Response;
// use warp::http::StatusCode;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct AdventuresQueryReq {
    pub item_id: u8,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl From<AdventuresQueryReq> for domain::AdventuresQuery {
    fn from(ad: AdventuresQueryReq) -> Self {
        Self {
            item_id: ad.item_id,
            limit: ad.limit,
            offset: ad.offset,
        }
    }
}

pub async fn list_adventures(
    token: Option<String>,
    query: AdventuresQueryReq,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, query: {:?}, state: {:?}", token, query, state);
    let repository = &state.repository;
    let adventures = repository.find_adventures(query.into()).await.unwrap();
    let response = AdventuresResponse::from(adventures);

    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
