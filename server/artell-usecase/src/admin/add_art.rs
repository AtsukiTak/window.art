use artell_domain::{
    art::{Art, ArtRepository, Error as ArtDomainError},
    artist::ArtistRepository,
    image::{Error as ImageDomainError, Image, ImageRepository},
};
use bytes::Bytes;
use uuid::Uuid;

pub struct Params {
    pub artist_id: Uuid,
    pub title: String,
    pub materials: String,
    // (width, height)
    pub size: Option<(usize, usize)>,
    pub image_data: Bytes,
    pub portfolio_link: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("artist not found")]
    ArtistNotFound,
    #[error("image domain invariance violation")]
    ImageDomainViolation(#[from] ImageDomainError),
    #[error("art domain invariance violation")]
    ArtDomainViolation(#[from] ArtDomainError),
    #[error(transparent)]
    Others(#[from] anyhow::Error),
}

pub async fn admin_add_art(
    params: Params,
    artist_repo: impl ArtistRepository,
    art_repo: impl ArtRepository,
    image_repo: impl ImageRepository,
) -> Result<Uuid, Error> {
    // Artistが存在することを確認
    let artist = artist_repo
        .find_by_id(params.artist_id)
        .await?
        .ok_or(Error::ArtistNotFound)?;

    // imageを保存
    let image = Image::new(params.image_data)?;
    let image_name = image.name.clone();
    image_repo.save(image).await?;

    // artを作成、保存
    let art = Art::new(
        artist.id,
        params.title,
        params.materials,
        params.size,
        image_name,
        params.portfolio_link,
    )?;
    let art_id = art.id;
    art_repo.save(art).await?;

    Ok(art_id.0)
}
