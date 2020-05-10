use super::{schema::arts, Connection, Postgres};
use artell_domain::{
    art::{Art, ArtId, ArtRepository},
    artist::ArtistId,
};
use diesel::prelude::*;
use uuid::Uuid;

pub struct PgArtRepository {
    pg: Postgres,
}

impl PgArtRepository {
    pub fn new(pg: Postgres) -> Self {
        PgArtRepository { pg }
    }
}

#[async_trait]
impl ArtRepository for PgArtRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Art>> {
        self.pg
            .try_with_conn(move |conn| find_by_id(conn, id))
            .await
    }

    async fn save(&self, art: Art) -> anyhow::Result<()> {
        self.pg.try_with_conn(move |conn| save(conn, art)).await
    }
}

/*
 * ========
 * Query
 * ========
 */
#[derive(Queryable)]
struct QueriedArt {
    id: Uuid,
    artist_id: Uuid,
    title: String,
    image_name: String,
}

impl Into<Art> for QueriedArt {
    fn into(self) -> Art {
        Art {
            id: ArtId(self.id),
            artist_id: ArtistId(self.artist_id),
            title: self.title,
            image_name: self.image_name,
        }
    }
}

fn find_by_id(conn: Connection, id: Uuid) -> anyhow::Result<Option<Art>> {
    Ok(arts::table
        .filter(arts::id.eq(id))
        .select((arts::id, arts::artist_id, arts::title, arts::image_name))
        .first::<QueriedArt>(&conn)
        .optional()?
        .map(QueriedArt::into))
}

/*
 * ========
 * Update
 * ========
 */
#[derive(Clone, Copy, Insertable, AsChangeset)]
#[table_name = "arts"]
struct NewArt<'a> {
    id: &'a Uuid,
    artist_id: &'a Uuid,
    title: &'a str,
    image_name: &'a str,
}

impl<'a> From<&'a Art> for NewArt<'a> {
    fn from(art: &'a Art) -> NewArt<'a> {
        NewArt {
            id: &art.id.0,
            artist_id: &art.artist_id.0,
            title: art.title.as_str(),
            image_name: art.image_name.as_str(),
        }
    }
}

fn save(conn: Connection, art: Art) -> anyhow::Result<()> {
    let new_art = NewArt::from(&art);
    diesel::insert_into(arts::table)
        .values(new_art)
        .on_conflict(arts::id)
        .do_update()
        .set(new_art)
        .execute(&conn)?;
    Ok(())
}
