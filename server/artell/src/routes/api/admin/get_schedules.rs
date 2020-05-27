use crate::{handler_fn, response_ok, Config, Error, Response};
use artell_usecase::admin::get_schedules as usecase;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use warp::{reject::Rejection, Filter};

#[derive(Serialize, Deserialize)]
pub struct ResBody {
    pub schedules: Vec<ResSchedule>,
}

#[derive(Serialize, Deserialize)]
pub struct ResSchedule {
    pub art_id: Uuid,
    pub activate_at: DateTime<Utc>,
}

pub fn route(config: Config) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admin" / "get_schedules")
        .and(warp::filters::method::get())
        .and(config.as_filter())
        .and_then(|config| handler_fn(move || handler(config)))
        .or_else(Error::recover)
}

async fn handler(config: Config) -> Result<Response, Error> {
    let scheduler = usecase::execute(config.scheduler_repo())
        .await
        .map_err(|e| match e {
            usecase::Error::SchedulerNotInitialized => {
                Error::internal_server_error("scheduler is not initialized")
            }
            usecase::Error::Others(_) => Error::internal_server_error("server error"),
        })?;

    Ok(response_ok(&ResBody {
        schedules: scheduler
            .schedules
            .into_iter()
            .map(|schedule| ResSchedule {
                art_id: schedule.art_id.0,
                activate_at: schedule.activate_at,
            })
            .collect(),
    }))
}
