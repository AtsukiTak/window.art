use crate::{api, html};
use std::net::SocketAddr;
use warp::{filters::log, Filter};

pub async fn bind(
    socket: impl Into<SocketAddr> + 'static,
    html_path: impl Into<std::path::PathBuf> + 'static,
) {
    let filter = api::filter()
        .or(html::filter(html_path))
        .with(log::log("crop"));

    let server = warp::serve(filter);
    server.bind(socket).await
}
