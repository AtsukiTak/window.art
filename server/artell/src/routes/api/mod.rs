pub mod admin;
pub mod user;

use crate::Config;
use warp::{filters::path, Filter, Rejection, Reply};

/// Helper to combine the multiple filters together with Filter::or, possibly boxing the types in
/// the process. This greatly helps the build times for `ipfs-http`.
macro_rules! combine {
    ($x:expr, $($y:expr),+) => {
        {
            let filter = boxed_on_debug!($x);
            $(
                let filter = boxed_on_debug!(filter.or($y));
            )+
            filter
        }
    };
}

#[cfg(debug_assertions)]
macro_rules! boxed_on_debug {
    ($x:expr) => {
        $x.boxed()
    };
}

#[cfg(not(debug_assertions))]
macro_rules! boxed_on_debug {
    ($x:expr) => {
        $x
    };
}

pub fn route(config: Config) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let mount = path::path("api").and(path::path("v1"));

    let cors = warp::filters::cors::cors()
        .allow_any_origin()
        .allow_methods(vec!["POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"]);

    let routes = combine!(
        user::get_current_art::route(config.clone()),
        admin::get_schedules::route(config.clone()),
        admin::add_art::route(config.clone()),
        admin::add_schedule::route(config.clone()),
        admin::add_artist::route(config.clone())
    );

    mount.and(routes).with(cors)
}
