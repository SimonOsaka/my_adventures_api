use crate::app::AppState;
use crate::handlers::adventures::responses::InsertResponse;
use domain::repositories::Repository;
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct AdventuresInsertReq {
    pub title: String,
    pub image_url: String,
}

impl From<AdventuresInsertReq> for domain::AdventureContent {
    fn from(req: AdventuresInsertReq) -> Self {
        Self {
            title: req.title,
            image_url: req.image_url,
        }
    }
}

pub async fn insert_adventure(
    token: Option<String>,
    body: AdventuresInsertReq,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, body: {:?}, state: {:?}", token, body, state);
    let repository = &state.repository;
    let id: u64 = repository.insert_adventure(body.into()).await.unwrap();

    let response = InsertResponse { id: id };

    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
