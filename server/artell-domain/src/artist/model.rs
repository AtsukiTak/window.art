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

#[derive(Error, Debug)]
pub enum Error {
    #[error("name must not be empty")]
    EmptyName,
    #[error("email must not be empty")]
    EmptyEmail,
}

impl Artist {
    pub fn new(
        name: String,
        email: String,
        status_msg: String,
        description: String,
        instagram: String,
        twitter: String,
    ) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }

        if email.is_empty() {
            return Err(Error::EmptyEmail);
        }

        Ok(Artist {
            id: ArtistId::new(),
            name,
            email,
            status_msg,
            description,
            instagram,
            twitter,
        })
    }

    pub fn update_name(&mut self, name: String) -> Result<(), Error> {
        if name.is_empty() {
            return Err(Error::EmptyName);
        }

        self.name = name;

        Ok(())
    }

    pub fn update_status_msg(&mut self, status_msg: String) {
        self.status_msg = status_msg;
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
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

impl AsRef<Uuid> for ArtistId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}
