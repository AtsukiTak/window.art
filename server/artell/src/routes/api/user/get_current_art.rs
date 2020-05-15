use crate::{handler_fn, response_ok, Config, Error, Response};
use artell_usecase::user::get_current_art as usecase;
use uuid::Uuid;
use warp::{reject::Rejection, Filter};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResBody<'a> {
    art_id: &'a Uuid,
    art_title: &'a str,
    art_materials: &'a str,
    art_size: Option<(usize, usize)>,
    artist_name: &'a str,
    image_url: &'a str,
    portfolio_id: &'a str,
}

pub fn route(config: Config) -> impl Filter<Extract = (Response,), Error = Rejection> + Clone {
    warp::path!("user" / "get_current_art")
        .and(warp::filters::method::get())
        .and(config.as_filter())
        .and_then(|config| handler_fn(move || handler(config)))
        .or_else(Error::recover)
}

async fn handler(config: Config) -> Result<Response, Error> {
    let current_art = usecase::get_current_art(
        config.scheduler_repo(),
        config.art_repo(),
        config.artist_repo(),
        config.image_repo(),
    )
    .await?
    .ok_or_else(|| Error::not_found("current art is not found"))?;

    Ok(response_ok(&ResBody {
        art_id: &current_art.art.id.0,
        art_title: current_art.art.title.as_str(),
        art_materials: current_art.art.materials.as_str(),
        art_size: current_art.art.size.map(|size| (size.width, size.height)),
        artist_name: &current_art.artist.name.as_str(),
        image_url: current_art.image_url.as_str(),
        portfolio_id: current_art.art.portfolio_id.as_str(),
    }))
}
