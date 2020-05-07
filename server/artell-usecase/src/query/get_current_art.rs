use artell_domain::{
    art::{Art, ArtRepository},
    scheduler::SchedulerRepository,
};

/// TODO
/// Authorization
pub async fn get_current_art(
    scheduler_repo: impl SchedulerRepository,
    art_repo: impl ArtRepository,
) -> anyhow::Result<Option<Art>> {
    let scheduler = scheduler_repo
        .find()
        .await?
        .ok_or_else(|| anyhow::anyhow!("Scheduler is not initialized"))?;

    if let Some(art_id) = scheduler.current_art_id().copied() {
        let art = art_repo.find_by_id(art_id.0).await?.expect("Infallible");

        Ok(Some(art))
    } else {
        Ok(None)
    }
}
