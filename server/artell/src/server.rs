use crate::routes;
use std::net::SocketAddr;
use warp::{filters::log, Filter};

pub async fn bind(socket: impl Into<SocketAddr> + 'static) {
    let filter = routes::api::route().with(log::log("crop"));

    let server = warp::serve(filter);
    server.bind(socket).await
}
