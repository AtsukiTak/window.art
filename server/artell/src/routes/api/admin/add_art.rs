use crate::res::{handler_fn, response, Error, Response};
use artell_infra::{
    pg::{GlobalPostgres, PgArtRepository, PgArtistRepository},
    s3::S3ImageRepository,
};
use artell_usecase::command::admin_add_art as usecase;
use bytes::Bytes;
use http::StatusCode;
use uuid::Uuid;
use warp::{reject::Rejection, Filter};

#[derive(Deserialize)]
pub struct ReqBody {
    artist_id: Uuid,
    title: String,
    // base64 encoded
    image_data: String,
}

pub fn route() -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admin" / "add_art")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(|body| handler_fn(move || handler(body)))
        .or_else(Error::recover)
}

async fn handler(body: ReqBody) -> Result<Response, Error> {
    let image_bytes = decode_base64(body.image_data.as_str())?;

    let artist_repo = PgArtistRepository::new(GlobalPostgres::get());
    let art_repo = PgArtRepository::new(GlobalPostgres::get());
    let image_repo = S3ImageRepository::new("artell".to_string());

    let params = usecase::Params {
        artist_id: body.artist_id,
        title: body.title,
        image_data: image_bytes,
    };

    usecase::admin_add_art(params, artist_repo, art_repo, image_repo)
        .await
        .map_err(|e| match e {
            usecase::Error::ArtistNotFound => {
                Error::new(StatusCode::BAD_REQUEST, "artist not found")
            }
            usecase::Error::ImageDomainViolation(_) => {
                Error::new(StatusCode::BAD_REQUEST, "invalid argument")
            }
            usecase::Error::ArtDomainViolation(_) => {
                Error::new(StatusCode::BAD_REQUEST, "invalid argument")
            }
            usecase::Error::Others(_) => {
                Error::new(StatusCode::INTERNAL_SERVER_ERROR, "server error")
            }
        })
        .map(|artist_id| response(StatusCode::OK, &artist_id))
}

fn decode_base64(s: &str) -> Result<Bytes, Error> {
    base64::decode(s.as_bytes())
        .map(|bytes| Bytes::from(bytes))
        .map_err(|e| {
            log::warn!("base64 decode error. {:?}", e);
            Error::new(StatusCode::BAD_REQUEST, "failed to decode base64 data")
        })
}
