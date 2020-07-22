use crate::app::AppState;
use crate::auth::decode_token;
use crate::response::Response;

// use crate::handlers::profiles::responses::ProfileResponse;
use domain::repositories::Repository;

use std::convert::Infallible;
use warp::http::StatusCode;

pub enum Action {
    Follow,
    Unfollow,
}

pub async fn follow(username: String, token: String, state: AppState) -> Result<impl warp::Reply, Infallible> {
    _follow(username, token, state, Action::Follow).await
}

pub async fn unfollow(username: String, token: String, state: AppState) -> Result<impl warp::Reply, Infallible> {
    _follow(username, token, state, Action::Unfollow).await
}

async fn _follow(username: String, token: String, state: AppState, action: Action) -> Result<impl warp::Reply, Infallible> {
    // let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    // let repository = &state.repository;

    // let user = repository.get_user_by_id(user_id)?;
    // let profile = repository.get_profile(&username)?;
    // let view = match action {
    //     Action::Follow => user.follow(profile, repository)?,
    //     Action::Unfollow => user.unfollow(profile, repository)?,
    // };

    // let response = ProfileResponse::from(view);
    // Ok(warp::reply::json(&response))
    Ok(StatusCode::OK)
}
