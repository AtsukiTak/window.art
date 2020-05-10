use super::Image;

#[async_trait]
pub trait ImageRepository {
    fn url_to(&self, image_name: &str) -> String;

    async fn save(&self, image: Image) -> anyhow::Result<()>;
}
