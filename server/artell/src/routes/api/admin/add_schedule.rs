use crate::res::{handler_fn, response, Error, Response};
use artell_infra::pg::{GlobalPostgres, PgArtRepository, PgSchedulerRepository};
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

pub fn route() -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admin" / "add_schedule")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(|body| handler_fn(move || handler(body)))
        .or_else(Error::recover)
}

async fn handler(body: ReqBody) -> Result<Response, Error> {
    let art_repo = PgArtRepository::new(GlobalPostgres::get());
    let scheduler_repo = PgSchedulerRepository::new(GlobalPostgres::get());

    let params = usecase::Params {
        art_id: body.art_id,
        activate_at: body.activate_at,
    };

    usecase::admin_add_schedule(params, art_repo, scheduler_repo)
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
