use super::Image;

#[async_trait]
pub trait ImageRepository {
    fn url_to(&self, image: &Image) -> String;

    async fn save(&self, image: Image) -> anyhow::Result<()>;
}
