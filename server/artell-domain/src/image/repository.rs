use super::Image;
use uuid::Uuid;

#[async_trait]
pub trait ImageRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Image>>;

    async fn save(&self, image: Image) -> anyhow::Result<()>;
}
