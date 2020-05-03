use crate::artist::ArtistId;
use uuid::Uuid;

pub struct Art {
    pub id: ArtId,
    pub artist_id: ArtistId,
    pub title: String,
}

pub struct ArtId(pub Uuid);
