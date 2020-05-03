use super::{Art, ArtId};

#[async_trait]
pub trait ArtRepository {
    async fn find_by_id(&self, id: ArtId) -> anyhow::Result<Option<Art>>;

    async fn save(&self, art: Art) -> anyhow::Result<()>;
}
