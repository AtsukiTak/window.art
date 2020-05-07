use super::{Image, ImageId};
use std::path::PathBuf;

#[async_trait]
pub trait ImageRepository {
    fn path_to(&self, image_id: &ImageId) -> PathBuf;

    async fn save(&self, image: Image) -> anyhow::Result<()>;
}
