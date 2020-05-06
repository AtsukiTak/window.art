use crate::{artist::ArtistId, image::ImageId};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Art {
    pub id: ArtId,
    pub artist_id: ArtistId,
    pub title: String,
    pub image_id: ImageId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtId(pub Uuid);

impl std::fmt::Display for ArtId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid argument `{0}`")]
    InvalidArgument(&'static str),
}

impl Art {
    // Artistがちゃんと存在しているという制約はどこに入れる？
    // Application Service?
    pub fn new(artist_id: ArtistId, title: String, image_id: ImageId) -> Result<Self, Error> {
        if title.is_empty() {
            return Err(Error::InvalidArgument("title"));
        }

        Ok(Art {
            id: ArtId(Uuid::new_v4()),
            artist_id,
            title,
            image_id,
        })
    }
}
