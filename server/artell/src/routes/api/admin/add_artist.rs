use crate::{handler_fn, response, Config, Error, Response};
use artell_usecase::admin::add_artist::{admin_add_artist, Error as AppError, Params};
use http::StatusCode;
use warp::{reject::Rejection, Filter};

#[derive(Deserialize)]
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
            AppError::ArtistDomainInvarianceViolation(_) => {
                Error::new(StatusCode::BAD_REQUEST, "invalid argument")
            }
            AppError::Others(_) => Error::new(StatusCode::INTERNAL_SERVER_ERROR, "server error"),
        })
        .map(|artist_id| response(StatusCode::OK, &artist_id.0))
}
