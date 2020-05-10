use crate::{routes, Config};
use artell_infra::pg::{GlobalPostgres, PgSchedulerRepository};
use artell_usecase::system::check_scheduler::system_update_scheduler;
use chrono::{Timelike, Utc};
use futures::{future, StreamExt};
use std::net::SocketAddr;
use tokio::time::{interval_at, Duration, Instant, Interval};
use warp::Filter;

pub async fn bind(config: Config, socket: impl Into<SocketAddr> + 'static) {
    let filter = routes::api::route(config).with(warp::filters::log::log("crop"));

    let server = warp::serve(filter);
    let server_fut = server.bind(socket);

    let cron_fut = start_system_cron();
    tokio::pin!(cron_fut);

    future::select(server_fut, cron_fut).await;
}

async fn start_system_cron() {
    system_cron_stream().for_each(|_| update_scheduler()).await
}

fn system_cron_stream() -> Interval {
    let interval_dur = Duration::from_secs(60 * 60);

    let now = Utc::now();
    let mins_to_next_oclock = (60 - now.minute() as u64) % 60;
    let dur_to_next_oclock = Duration::from_secs(mins_to_next_oclock * 60);
    let next_oclock = Instant::now() + dur_to_next_oclock;

    interval_at(next_oclock, interval_dur)
}

async fn update_scheduler() {
    let scheduler_repo = PgSchedulerRepository::new(GlobalPostgres::get());

    if let Err(e) = system_update_scheduler(scheduler_repo).await {
        log::error!("{:?}", e);
    }
}
