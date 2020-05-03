mod artist;
mod postgres;
#[allow(unused_imports)]
mod schema;

pub use artist::PgArtistRepository;
pub use postgres::{GlobalPostgres, Postgres, GLOBAL_PG};
