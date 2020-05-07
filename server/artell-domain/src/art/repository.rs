use super::Art;
use uuid::Uuid;

#[async_trait]
pub trait ArtRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Art>>;

    async fn save(&self, art: Art) -> anyhow::Result<()>;
}
