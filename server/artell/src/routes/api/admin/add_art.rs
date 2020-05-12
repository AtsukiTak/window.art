use crate::{handler_fn, response_ok, Config, Error, Response};
use artell_usecase::admin::add_art as usecase;
use bytes::Bytes;
use uuid::Uuid;
use warp::{reject::Rejection, Filter};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqBody {
    artist_id: Uuid,
    title: String,
    // base64 encoded
    image_data: String,
}

pub fn route(config: Config) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admin" / "add_art")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and(config.as_filter())
        .and_then(|body, config| handler_fn(move || handler(config, body)))
        .or_else(Error::recover)
}

async fn handler(config: Config, body: ReqBody) -> Result<Response, Error> {
    let image_bytes = decode_base64(body.image_data.as_str())?;

    let params = usecase::Params {
        artist_id: body.artist_id,
        title: body.title,
        image_data: image_bytes,
    };

    usecase::admin_add_art(
        params,
        config.artist_repo(),
        config.art_repo(),
        config.image_repo(),
    )
    .await
    .map_err(|e| match e {
        usecase::Error::ArtistNotFound => Error::bad_request("artist not found"),
        usecase::Error::ImageDomainViolation(_) => Error::bad_request("invalid argument"),
        usecase::Error::ArtDomainViolation(_) => Error::bad_request("invalid argument"),
        usecase::Error::Others(e) => Error::from(e),
    })
    .map(|artist_id| response_ok(&artist_id))
}

fn decode_base64(s: &str) -> Result<Bytes, Error> {
    base64::decode(s.as_bytes())
        .map(|bytes| Bytes::from(bytes))
        .map_err(|e| {
            log::warn!("base64 decode error. {:?}", e);
            Error::bad_request("failed to decode base64 data")
        })
}
