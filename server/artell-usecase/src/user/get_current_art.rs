use artell_domain::{
    art::{Art, ArtRepository},
    artist::{Artist, ArtistRepository},
    image::ImageRepository,
    scheduler::SchedulerRepository,
};

pub struct Res {
    pub art: Art,
    pub artist: Artist,
    pub image_url: String,
}

/// TODO
/// Authorization
pub async fn get_current_art(
    scheduler_repo: impl SchedulerRepository,
    art_repo: impl ArtRepository,
    artist_repo: impl ArtistRepository,
    image_repo: impl ImageRepository,
) -> anyhow::Result<Option<Res>> {
    let scheduler = scheduler_repo
        .find()
        .await?
        .ok_or_else(|| anyhow::anyhow!("Scheduler is not initialized"))?;

    if let Some(art_id) = scheduler.current_art_id().copied() {
        let art = art_repo.find_by_id(art_id.0).await?.expect("Infallible");
        let artist = artist_repo
            .find_by_id(art.artist_id.0)
            .await?
            .expect("Infallible");
        let image_url = image_repo.url_to(art.image_name.as_str());

        Ok(Some(Res {
            art,
            artist,
            image_url,
        }))
    } else {
        Ok(None)
    }
}
