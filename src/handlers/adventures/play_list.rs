use crate::app::AppState;
use crate::handlers::adventures::responses::AdventuresResponse;
use domain::repositories::Repository;
use std::convert::Infallible;

pub async fn play_list_adventures(
    play_list: String,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, play_list: {:?}, state: {:?}", token, play_list, state);
    let repository = &state.repository;
    let query = domain::PlayListQuery { play_list };
    let adventures = repository.find_adventures_by_play_list(query).await.unwrap();
    let response = AdventuresResponse::from(adventures);

    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
