use super::{Artist, ArtistId};

#[async_trait]
pub trait ArtistRepository {
    async fn find_by_id(&self, id: ArtistId) -> anyhow::Result<Artist>;

    async fn save(&self, artist: Artist) -> anyhow::Result<()>;
}
