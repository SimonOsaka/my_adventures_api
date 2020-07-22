use crate::app::AppState;
use crate::response::Response;

use crate::auth::{decode_token, encode_token};
// use crate::handlers::users::responses::UserResponse;
use domain::repositories::Repository;

use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn get_current_user(token: String, state: AppState) -> Result<impl warp::Reply, Infallible> {
    // let user_id = decode_token(&state.jwt_secret, &token)?.user_id();
    // let repository = &state.repository;

    // let user = repository.get_user_by_id(user_id)?;
    // let token = encode_token(&state.jwt_secret, user.id);

    // let response = UserResponse::from((user, token));

    // Ok(warp::reply::json(&response))

    Ok(StatusCode::OK)
}
