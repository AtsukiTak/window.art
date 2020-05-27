use artell_domain::scheduler::{Scheduler, SchedulerRepository};

#[derive(Debug, Error)]
pub enum Error {
    #[error("scheduler is not initialized yet")]
    SchedulerNotInitialized,
    #[error(transparent)]
    Others(#[from] anyhow::Error),
}

pub async fn admin_add_schedule(
    scheduler_repo: impl SchedulerRepository,
) -> Result<Scheduler, Error> {
    let scheduler = scheduler_repo
        .find()
        .await?
        .ok_or(Error::SchedulerNotInitialized)?;

    Ok(scheduler)
}
