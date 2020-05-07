use super::Artist;
use uuid::Uuid;

#[async_trait]
pub trait ArtistRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Artist>>;

    async fn save(&self, artist: Artist) -> anyhow::Result<()>;
}
