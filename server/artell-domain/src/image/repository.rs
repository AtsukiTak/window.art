use super::{Image, ImageId};

#[async_trait]
pub trait ImageRepository {
    async fn find_by_id(&self, id: ImageId) -> anyhow::Result<Option<Image>>;

    async fn save(&self, image: Image) -> anyhow::Result<()>;
}
