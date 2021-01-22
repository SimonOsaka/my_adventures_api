use crate::app::AppState;
use crate::handlers::adventures::responses::DeletedResponse;
use domain::repositories::Repository;
use std::convert::Infallible;

pub async fn delete_adventure(
    _id: u64,
    token: Option<String>,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, _id: {:?}, state: {:?}", token, _id, state);
    let repository = &state.repository;
    let deleted = repository.delete_adventure(_id).await.unwrap();
    let response = DeletedResponse { id: _id, deleted };

    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
