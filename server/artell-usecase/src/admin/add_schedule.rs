use artell_domain::{art::ArtRepository, scheduler::SchedulerRepository};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Params {
    pub art_id: Uuid,
    pub activate_at: DateTime<Utc>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("art {0} is not found")]
    ArtNotFound(Uuid),
    #[error("scheduler is not initialized yet")]
    SchedulerNotInitialized,
    #[error(transparent)]
    Others(#[from] anyhow::Error),
}

pub async fn admin_add_schedule(
    params: Params,
    art_repo: impl ArtRepository,
    scheduler_repo: impl SchedulerRepository,
) -> Result<(), Error> {
    let Params {
        art_id,
        activate_at,
    } = params;

    // Artが存在することを確認
    let art = art_repo
        .find_by_id(art_id)
        .await?
        .ok_or(Error::ArtNotFound(art_id))?;

    let mut scheduler = scheduler_repo
        .find()
        .await?
        .ok_or(Error::SchedulerNotInitialized)?;

    scheduler.add_schedule(art.id, activate_at);
    scheduler.check_update();

    scheduler_repo.save(scheduler).await?;

    Ok(())
}
