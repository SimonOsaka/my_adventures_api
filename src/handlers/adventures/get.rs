use crate::app::AppState;
use crate::handlers::adventures::responses::{AdventureResponse, Response404};
use domain::repositories::Repository;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::Reply;

pub async fn get_adventure(
    _id: u64,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);
    let repository = &state.repository;
    if let Some(adventure) = repository.get_adventure_by_id(_id).await.unwrap() {
        let response = AdventureResponse::from(adventure);
        debug!("response: {:?}", &response);
        Ok(warp::reply::json(&response).into_response())
    } else {
        let json = warp::reply::json(&Response404 { message: "404".to_owned() });
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND).into_response())
    }
}
