use super::{schema::artists, Postgres};
use artell_domain::artist::{Artist, ArtistId, ArtistRepository};
use diesel::prelude::*;
use uuid::Uuid;

pub struct PgArtistRepository {
    pg: Postgres,
}

#[async_trait]
impl ArtistRepository for PgArtistRepository {
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<Artist>> {
        #[derive(Queryable)]
        struct QueriedArtist {
            id: Uuid,
            name: String,
            email: String,
            status_msg: String,
            description: String,
            instagram: String,
            twitter: String,
        }

        self.pg
            .try_with_conn(move |conn| {
                Ok(artists::table
                    .filter(artists::id.eq(id))
                    .select((
                        artists::id,
                        artists::name,
                        artists::email,
                        artists::status_msg,
                        artists::description,
                        artists::instagram,
                        artists::twitter,
                    ))
                    .first::<QueriedArtist>(&conn)
                    .optional()?
                    .map(|a| Artist {
                        id: ArtistId(a.id),
                        name: a.name,
                        email: a.email,
                        status_msg: a.status_msg,
                        description: a.description,
                        instagram: a.instagram,
                        twitter: a.twitter,
                    }))
            })
            .await
    }

    async fn save(&self, artist: Artist) -> anyhow::Result<()> {
        #[derive(Clone, Copy, Insertable, AsChangeset)]
        #[table_name = "artists"]
        struct NewArtist<'a> {
            id: &'a Uuid,
            name: &'a str,
            email: &'a str,
            status_msg: &'a str,
            description: &'a str,
            instagram: &'a str,
            twitter: &'a str,
        }

        self.pg
            .try_with_conn(move |conn| {
                let new_artist = NewArtist {
                    id: artist.id.as_ref(),
                    name: artist.name.as_str(),
                    email: artist.email.as_str(),
                    status_msg: artist.status_msg.as_str(),
                    description: artist.description.as_str(),
                    instagram: artist.instagram.as_str(),
                    twitter: artist.twitter.as_str(),
                };
                diesel::insert_into(artists::table)
                    .values(new_artist)
                    .on_conflict(artists::id)
                    .do_update()
                    .set(new_artist)
                    .execute(&conn)?;
                Ok(())
            })
            .await
    }
}
