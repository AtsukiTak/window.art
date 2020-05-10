use crate::res::{handler_fn, response, Error, Response};
use artell_infra::{
    pg::{GlobalPostgres, PgArtRepository, PgSchedulerRepository},
    s3::S3ImageRepository,
};
use artell_usecase::user::get_current_art as usecase;
use http::StatusCode;
use uuid::Uuid;
use warp::{reject::Rejection, Filter};

#[derive(Serialize)]
pub struct ResBody<'a> {
    art_id: &'a Uuid,
    art_title: &'a str,
    artist_id: &'a Uuid,
    image_url: &'a str,
}

pub fn route() -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("user" / "get_current_art")
        .and(warp::filters::method::get())
        .and_then(|| handler_fn(handler))
        .or_else(Error::recover)
}

async fn handler() -> Result<Response, Error> {
    let art_repo = PgArtRepository::new(GlobalPostgres::get());
    let scheduler_repo = PgSchedulerRepository::new(GlobalPostgres::get());
    let image_repo = S3ImageRepository::new("artell".to_string());

    let current_art = usecase::get_current_art(scheduler_repo, art_repo, image_repo)
        .await?
        .ok_or_else(|| Error::new(StatusCode::NOT_FOUND, "current art is not found"))?;

    Ok(response(
        StatusCode::OK,
        &ResBody {
            art_id: &current_art.art.id.0,
            art_title: current_art.art.title.as_str(),
            artist_id: &current_art.art.artist_id.0,
            image_url: current_art.image_url.as_str(),
        },
    ))
}
