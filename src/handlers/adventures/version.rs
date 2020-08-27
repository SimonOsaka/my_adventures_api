use crate::handlers::adventures::responses::VersionUpdateResponse;
use serde::Deserialize;
use std::convert::Infallible;

#[derive(Default, Deserialize, Debug, Clone)]
pub struct VersionUpdateReq {
    pub appid: String,
    pub version: String,
}

pub async fn version_update_adventures(
    token: Option<String>,
    query: VersionUpdateReq,
) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}, query: {:?}", token, query);

    if query.appid != "__UNI__410C039" && query.appid != "HBuilder" {
        let response = VersionUpdateResponse {
            is_update: false,
            note: None,
            i_os: None,
            android: None,
        };
        Ok(warp::reply::json(&response))
    } else {
        let mut is_update: bool = false;
        let mut note: Option<String> = None;
        let mut android: Option<String> = None;
        if query.version != "1.1.0" {
            is_update = true;
            note = Some("有新版本需要更新".to_string());
            android = Some("http://dl.jicu.vip/adventures_20200827.apk".to_string());
        }

        let response = VersionUpdateResponse {
            is_update,
            note,
            i_os: None,
            android,
        };
        debug!("response: {:?}", &response);
        Ok(warp::reply::json(&response))
    }
}
