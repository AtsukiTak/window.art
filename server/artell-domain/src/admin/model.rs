use crate::commons::{access_token::AccessToken, cred::Cred};
use chrono::Duration;
use uuid::Uuid;

pub struct Admin {
    pub id: AdminId,
    pub email: String,
    pub cred: Cred,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AdminId(pub Uuid);

#[derive(Error, Debug)]
pub enum Error {
    #[error("email must not be empty")]
    EmptyEmail,
    #[error("invalid password")]
    InvalidPassword,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl Admin {
    pub fn new(email: String, pass: &str) -> Result<Self, Error> {
        if email.is_empty() {
            return Err(Error::EmptyEmail);
        }

        let cred = Cred::derive(pass)?;

        Ok(Admin {
            id: AdminId(Uuid::new_v4()),
            email,
            cred,
        })
    }

    pub fn publish_access_token(
        &self,
        pass: &str,
        exp: Duration,
    ) -> Result<AccessToken<AdminId>, Error> {
        self.cred.verify(pass).map_err(|_| Error::InvalidPassword)?;

        Ok(AccessToken::new(self.id, exp))
    }
}
