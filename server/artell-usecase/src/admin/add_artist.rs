use artell_domain::artist::{
    ArtistId, ArtistRepository, {Artist, Error as ArtistDomainError},
};

pub struct Params {
    pub name: String,
    pub email: String,
    pub status_msg: String,
    pub description: String,
    pub instagram: String,
    pub twitter: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("violate artist domain invariance rule. {0}")]
    ArtistDomainInvarianceViolation(#[from] ArtistDomainError),
    #[error(transparent)]
    Others(#[from] anyhow::Error),
}

/// TODO
/// 認証
pub async fn admin_add_artist(
    params: Params,
    artist_repo: impl ArtistRepository,
) -> Result<ArtistId, Error> {
    let Params {
        name,
        email,
        status_msg,
        description,
        instagram,
        twitter,
    } = params;

    let artist = Artist::new(name, email, status_msg, description, instagram, twitter)?;

    let artist_id = artist.id;

    artist_repo.save(artist).await?;

    Ok(artist_id)
}
