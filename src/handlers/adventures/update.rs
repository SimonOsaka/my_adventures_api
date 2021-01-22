use crate::app::AppState;
use crate::handlers::adventures::responses::UpdatedResponse;
use domain::repositories::Repository;
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct AdventuresUpdateReq {
    pub id: u64,
    pub title: String,
    pub image_url: String,
}

impl From<AdventuresUpdateReq> for domain::AdventuresUpdate {
    fn from(req: AdventuresUpdateReq) -> Self {
        Self { id: req.id, title: req.title, image_url: req.image_url }
    }
}

pub async fn update_adventure(
    token: Option<String>,
    query: AdventuresUpdateReq,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, query: {:?}, state: {:?}", token, query, state);
    let repository = &state.repository;
    let id = query.id;
    let updated: bool = repository.update_adventure(query.into()).await.unwrap();
    let response = UpdatedResponse { id, updated };

    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
