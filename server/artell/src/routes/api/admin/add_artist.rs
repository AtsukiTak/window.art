use crate::res::{handler_fn, response, Error, Response};
use artell_infra::pg::{GlobalPostgres, PgArtistRepository};
use artell_usecase::command::admin_add_artist::{admin_add_artist, Error as AppError, Params};
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

pub fn route() -> impl Filter<Extract = (Response,), Error = Rejection> {
    warp::path!("admin" / "add_artist")
        .and(warp::filters::method::post())
        .and(warp::filters::body::json::<ReqBody>())
        .and_then(|body| handler_fn(move || handler(body)))
        .or_else(Error::recover)
}

async fn handler(body: ReqBody) -> Result<Response, Error> {
    let artist_repo = PgArtistRepository::new(GlobalPostgres::get());

    let params = Params {
        name: body.name,
        email: body.email,
        status_msg: body.status_msg,
        description: body.description,
        instagram: body.instagram,
        twitter: body.twitter,
    };

    admin_add_artist(params, artist_repo)
        .await
        .map_err(|e| match e {
            AppError::ArtistDomainInvarianceViolation(_) => {
                Error::new(StatusCode::BAD_REQUEST, "invalid argument")
            }
            AppError::Others(_) => Error::new(StatusCode::INTERNAL_SERVER_ERROR, "server error"),
        })
        .map(|artist_id| response(StatusCode::OK, &artist_id.0))
}
