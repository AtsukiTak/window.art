use artell_domain::scheduler::SchedulerRepository;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Others(#[from] anyhow::Error),
}

pub async fn system_update_scheduler(
    scheduler_repo: impl SchedulerRepository,
) -> Result<(), Error> {
    if let Some(mut scheduler) = scheduler_repo.find().await? {
        if scheduler.check_update() {
            scheduler_repo.save(scheduler).await?;
        }
    }

    Ok(())
}
