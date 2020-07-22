// use super::responses::UserResponse;
// use crate::app::AppState;
// use crate::auth::encode_token;
// use crate::response::Response;
// use domain::repositories::Repository;
// use domain::SignUp;
// use serde::Deserialize;
// use std::convert::{TryFrom, TryInto};

// use std::convert::Infallible;
// use warp::http::StatusCode;

// #[derive(Deserialize, Debug)]
// pub struct RegistrationRequest {
//     user: NewUserRequest,
// }

// #[derive(Deserialize, Debug)]
// pub struct NewUserRequest {
//     pub username: String,
//     pub email: String,
//     pub password: String,
// }

// impl TryFrom<RegistrationRequest> for SignUp {
//     type Error = domain::PasswordError;

//     fn try_from(r: RegistrationRequest) -> Result<Self, Self::Error> {
//         let sign_up = Self {
//             username: r.user.username,
//             password: domain::Password::from_clear_text(r.user.password)?,
//             email: r.user.email,
//         };
//         Ok(sign_up)
//     }
// }

// pub async fn register(form: RegistrationRequest, state: AppState) -> Result<impl warp::Reply, Infallible> {
//     // let repository = &state.repository;

//     // let sign_up: SignUp = form.try_into()?;
//     // let new_user = repository.sign_up(sign_up)?;
//     // let token = encode_token(&state.jwt_secret, new_user.id);

//     // let response = UserResponse::from((new_user, token));
//     // Ok(warp::reply::json(&response))

//     Ok(StatusCode::OK)
// }
