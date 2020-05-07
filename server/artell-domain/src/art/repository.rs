use super::Art;
use uuid::Uuid;

#[async_trait]
pub trait ArtRepository {
    /// ## Developper note
    /// ここの引数は `ArtId` ではなく、 `Uuid` を使用する。
    /// 理由は、 `ArtId` は実在するArtのidとして使いたいから。
    /// つまり `ArtId` が存在しているならば、 `Art` が
    /// 必ず存在している。
    /// ここでは存在しているかわからないArtを検索するので
    /// Uuidを使う。
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Art>>;

    async fn save(&self, art: Art) -> anyhow::Result<()>;
}
