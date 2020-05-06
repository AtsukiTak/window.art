use artell_domain::{
    art::{Art, ArtRepository, Error as ArtDomainError},
    artist::{ArtistId, ArtistRepository},
    image::{Error as ImageDomainError, Image, ImageRepository},
};
use bytes::Bytes;
use uuid::Uuid;

pub struct AddArtParams {
    pub artist_id: Uuid,
    pub title: String,
    pub image_data: Bytes,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("artist not found")]
    ArtistNotFound,
    #[error("image domain invariance violation")]
    ImageDomainInvariance(#[from] ImageDomainError),
    #[error("art domain invariance violation")]
    ArtDomainInvarianceViolation(#[from] ArtDomainError),
    #[error(transparent)]
    Others(#[from] anyhow::Error),
}

pub async fn add_art(
    params: AddArtParams,
    artist_repo: impl ArtistRepository,
    art_repo: impl ArtRepository,
    image_repo: impl ImageRepository,
) -> Result<Uuid, Error> {
    // Artistが存在することを確認
    artist_repo
        .find_by_id(ArtistId(params.artist_id))
        .await?
        .ok_or(Error::ArtistNotFound)?;

    // imageを保存
    let image = Image::new(params.image_data)?;
    let image_id = image.id;
    image_repo.save(image).await?;

    // artを作成、保存
    let art = Art::new(ArtistId(params.artist_id), params.title, image_id)?;
    let art_id = art.id;
    art_repo.save(art).await?;

    Ok(art_id.0)
}
