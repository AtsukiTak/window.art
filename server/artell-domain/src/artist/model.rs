use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    pub email: String,
    pub status_msg: String,
    pub description: String,
    pub instagram: String,
    pub twitter: String,
}

impl Artist {
    pub fn new(
        name: String,
        email: String,
        status_msg: String,
        description: String,
        instagram: String,
        twitter: String,
    ) -> Self {
        Artist {
            id: ArtistId::new(),
            name,
            email,
            status_msg,
            description,
            instagram,
            twitter,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArtistId(pub Uuid);

impl ArtistId {
    fn new() -> Self {
        ArtistId(Uuid::new_v4())
    }
}
