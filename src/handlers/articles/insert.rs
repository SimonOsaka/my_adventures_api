// use crate::app::AppState;
// use crate::auth::decode_token;
// use crate::handlers::articles::responses::ArticleResponse;
// use crate::response::Response;
// use domain::repositories::Repository;
// use serde::{Deserialize, Serialize};
// use std::convert::Infallible;
// use warp::http::StatusCode;

// #[derive(Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Request {
//     pub article: NewArticleRequest,
// }

// #[derive(Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct NewArticleRequest {
//     pub title: String,
//     pub description: String,
//     pub body: String,
//     pub tag_list: Option<Vec<String>>,
// }

// impl From<NewArticleRequest> for domain::ArticleContent {
//     fn from(a: NewArticleRequest) -> domain::ArticleContent {
//         domain::ArticleContent {
//             title: a.title,
//             description: a.description,
//             body: a.body,
//             tag_list: a.tag_list.unwrap_or_else(|| vec![]),
//         }
//     }
// }

// pub async fn insert_article(token: String, form: Request, state: AppState) -> Result<impl warp::Reply, Infallible> {

//     Ok(StatusCode::OK)
// }
