use crate::{handler_fn, response_ok, Config, Error, Response};
use artell_usecase::admin::add_artist::{admin_add_artist, Error as AppError, Params};
use warp::{reject::Rejection, Filter};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqBody {
    name: String,
    email: String,
    status_msg: String,
    description: String,
    instagram: String,
    twitter: String,
}

pub fn route(config: Config) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("admin" / "add_artist")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and(config.as_filter())
        .and_then(|body, config| handler_fn(move || handler(config, body)))
        .or_else(Error::recover)
}

async fn handler(config: Config, body: ReqBody) -> Result<Response, Error> {
    let params = Params {
        name: body.name,
        email: body.email,
        status_msg: body.status_msg,
        description: body.description,
        instagram: body.instagram,
        twitter: body.twitter,
    };

    admin_add_artist(params, config.artist_repo())
        .await
        .map_err(|e| match e {
            AppError::ArtistDomainInvarianceViolation(_) => Error::bad_request("invalid argument"),
            AppError::Others(e) => Error::from(e),
        })
        .map(|artist_id| response_ok(&artist_id.0))
}
