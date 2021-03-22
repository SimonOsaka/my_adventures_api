use std::env;
use std::net::SocketAddr;

use crate::routes;
use db::{connection::Repo, Repository};
use warp::http::HeaderValue;
use warp::{self, Filter};

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");
const IP_NONE: &str = "ip_none";

#[derive(Clone, Debug)]
pub struct AppStateRaw {
    pub repository: Repository,
    pub jwt_secret: String,
}

pub type AppState = std::sync::Arc<AppStateRaw>;

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let repository = Repository(Repo::new(&database_url).await);

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = std::sync::Arc::new(AppStateRaw { repository, jwt_secret });

    // custom log format
    // pass x-real-ip from nginx
    let log = warp::log::custom(|info| {
        info!(
            target: APPLICATION_NAME,
            "{:?} \"{} {} {:?}\" {} \"{}\" \"{}\" {:?}",
            info.request_headers().get("x-real-ip").unwrap_or(&HeaderValue::from_static(IP_NONE)),
            info.method(),
            info.path(),
            info.version(),
            info.status().as_u16(),
            info.referer().unwrap_or("-"),
            info.user_agent().unwrap_or("-"),
            info.elapsed(),
        );
    });

    let routes = routes::routes(app_state).with(log);

    println!("You can access the server at {}", bind_address);

    warp::serve(routes).run(bind_address).await;
}
