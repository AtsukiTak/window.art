use crate::{handler_fn, response_ok, Config, Error, Response};
use artell_usecase::admin::add_schedule as usecase;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use warp::{reject::Rejection, Filter};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqBody {
    art_id: Uuid,
    activate_at: DateTime<Utc>,
}

pub fn route(config: Config) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admin" / "add_schedule")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and(config.as_filter())
        .and_then(|body, config| handler_fn(move || handler(config, body)))
        .or_else(Error::recover)
}

async fn handler(config: Config, body: ReqBody) -> Result<Response, Error> {
    let params = usecase::Params {
        art_id: body.art_id,
        activate_at: body.activate_at,
    };

    usecase::admin_add_schedule(params, config.art_repo(), config.scheduler_repo())
        .await
        .map_err(|e| match e {
            usecase::Error::ArtNotFound(_) => Error::bad_request("art not found"),
            usecase::Error::SchedulerNotInitialized => {
                Error::internal_server_error("scheduler is not initialized")
            }
            usecase::Error::Others(_) => Error::internal_server_error("server error"),
        })?;

    Ok(response_ok(&()))
}
