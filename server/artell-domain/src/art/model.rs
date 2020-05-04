use crate::artist::ArtistId;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Art {
    pub id: ArtId,
    pub artist_id: ArtistId,
    pub title: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtId(pub Uuid);

impl std::fmt::Display for ArtId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
