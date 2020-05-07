use artell_domain::artist::{Artist, ArtistId, ArtistRepository};

/// TODO
/// 認証
pub async fn add_artist<AR>(
    name: String,
    email: String,
    status_msg: String,
    description: String,
    instagram: String,
    twitter: String,
    artist_repo: &AR,
) -> anyhow::Result<ArtistId>
where
    AR: ArtistRepository,
{
    let artist = Artist::new(name, email, status_msg, description, instagram, twitter)?;

    let artist_id = artist.id;

    artist_repo.save(artist).await?;

    Ok(artist_id)
}
