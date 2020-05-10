use artell_infra::{
    pg::{PgArtRepository, PgArtistRepository, PgSchedulerRepository, Postgres},
    s3::S3ImageRepository,
};
use std::convert::Infallible;
use warp::Filter;

#[derive(Clone)]
pub struct Config {
    pub pg: Postgres,
    pub s3_bucket: String,
}

impl Config {
    pub fn new(pg: Postgres, s3_bucket: String) -> Self {
        Config { pg, s3_bucket }
    }

    pub fn as_filter(self) -> impl Filter<Extract = (Config,), Error = Infallible> + Clone {
        warp::filters::any::any().map(move || self.clone())
    }

    pub fn art_repo(&self) -> PgArtRepository {
        PgArtRepository::new(self.pg.clone())
    }

    pub fn artist_repo(&self) -> PgArtistRepository {
        PgArtistRepository::new(self.pg.clone())
    }

    pub fn scheduler_repo(&self) -> PgSchedulerRepository {
        PgSchedulerRepository::new(self.pg.clone())
    }

    pub fn image_repo(&self) -> S3ImageRepository {
        S3ImageRepository::new(self.s3_bucket.clone())
    }
}
