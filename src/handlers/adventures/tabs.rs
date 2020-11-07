use crate::handlers::adventures::responses::{Tabs, TabsResponse};
use std::convert::Infallible;

pub async fn tabs_adventures(token: Option<String>) -> Result<impl warp::Reply, Infallible> {
    debug!("token: {:?}", token);

    let tabs: Vec<Tabs> = vec![
        Tabs {
            name: "全部".to_owned(),
            item_id: 0,
        },
        Tabs {
            name: "日常".to_owned(),
            item_id: 1,
        },
        Tabs {
            name: "搞笑".to_owned(),
            item_id: 2,
        },
        Tabs {
            name: "游戏".to_owned(),
            item_id: 3,
        },
        Tabs {
            name: "影视".to_owned(),
            item_id: 4,
        },
        Tabs {
            name: "旅游".to_owned(),
            item_id: 5,
        },
        Tabs {
            name: "美食".to_owned(),
            item_id: 6,
        },
    ];
    let response = TabsResponse { tab_list: tabs };
    debug!("response: {:?}", &response);
    Ok(warp::reply::json(&response))
}
