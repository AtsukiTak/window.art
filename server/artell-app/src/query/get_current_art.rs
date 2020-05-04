use artell_domain::{art::ArtRepository, scheduler::SchedulerRepository};

#[derive(Serialize)]
pub struct Res {
    title: String,
    image_url: String,
}

/// TODO
/// Authorization
pub async fn get_current_art<SR, AR>(scheduler_repo: &SR, art_repo: &AR) -> anyhow::Result<Res>
where
    SR: SchedulerRepository,
    AR: ArtRepository,
{
    let scheduler = scheduler_repo
        .find()
        .await?
        .ok_or_else(|| anyhow::anyhow!("Scheduler is not initialized"))?;

    let art = art_repo
        .find_by_id(scheduler.current_art_id)
        .await?
        .expect("Infallible");

    Ok(Res {
        title: art.title,
        image_url: format!("/images/arts/${}.png", art.id),
    })
}
