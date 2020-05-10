pub mod admin;
pub mod user;

use warp::{filters::path, Filter, Rejection, Reply};

/// Helper to combine the multiple filters together with Filter::or, possibly boxing the types in
/// the process. This greatly helps the build times for `ipfs-http`.
macro_rules! combine {
    ($x:expr) => {
        boxed_on_debug!($x)
    };
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

pub fn route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let mount = path::path("api").and(path::path("v1"));

    let routes = combine!(
        user::get_current_art::route(),
        admin::add_art::route(),
        admin::add_schedule::route(),
        admin::add_artist::route()
    );

    mount.and(routes)
}
