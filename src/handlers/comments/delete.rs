use crate::app::AppState;
use crate::auth::decode_token;
use crate::response::ErrorResponse;
use domain::repositories::Repository;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn delete(
    _slug: String,
    comment_id: u64,
    token: String,
    state: AppState,
) -> Result<impl warp::Reply, Infallible> {
    // let author_id = decode_token(&state.jwt_secret, &token)?.user_id();
    // let repository = &state.repository;

    // let author = repository.get_user_by_id(author_id)?;
    // let comment = repository.get_comment(comment_id)?;
    // author.delete_comment(comment, repository)?;

    // Ok(warp::http::status::StatusCode::OK)
    Ok(StatusCode::OK)
}
