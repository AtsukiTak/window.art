use super::Scheduler;

#[async_trait]
pub trait SchedulerRepository {
    async fn find(&self) -> anyhow::Result<Option<Scheduler>>;

    async fn save(&self, scheduler: Scheduler) -> anyhow::Result<()>;
}
