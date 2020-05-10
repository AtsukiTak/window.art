use crate::{handler_fn, response, Config, Error, Response};
use artell_usecase::admin::add_schedule as usecase;
use chrono::{DateTime, Utc};
use http::StatusCode;
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
            usecase::Error::ArtNotFound(_) => Error::new(StatusCode::BAD_REQUEST, "art not found"),
            usecase::Error::SchedulerNotInitialized => Error::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "scheduler is not initialized",
            ),
            usecase::Error::Others(_) => {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR, "server error")
            }
        })?;

    Ok(response(StatusCode::OK, &()))
}
