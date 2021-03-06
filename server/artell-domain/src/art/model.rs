use crate::artist::ArtistId;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Art {
    pub id: ArtId,
    pub artist_id: ArtistId,
    pub title: String,
    pub materials: String,
    pub size: Option<Size>,
    pub image_name: String,
    pub portfolio_link: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
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
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("portfolio_link must not be empty")]
    EmptyPortfolioLink,
}

impl Art {
    // Artistがちゃんと存在しているという制約はどこに入れる？
    // Application Service?
    //
    // ArtistIdが存在している => Artistが存在している
    // が成り立つようにする
    pub fn new(
        artist_id: ArtistId,
        title: String,
        materials: String,
        size: Option<(usize, usize)>, // (width, height)
        image_name: String,
        portfolio_link: String,
    ) -> Result<Self, Error> {
        if title.is_empty() {
            return Err(Error::EmptyTitle);
        }

        if portfolio_link.is_empty() {
            return Err(Error::EmptyPortfolioLink);
        }

        let size = size.map(|(width, height)| Size { width, height });

        Ok(Art {
            id: ArtId(Uuid::new_v4()),
            artist_id,
            title,
            materials,
            size,
            image_name,
            portfolio_link,
        })
    }
}
