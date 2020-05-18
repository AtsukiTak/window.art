use chrono::Duration;
use rego_domain::{
    access_token::{AccessToken, AccessTokenBody},
    cred::Cred,
};
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
}

/*
 * ===========
 * AccessToken
 * ===========
 */
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct AdminAccessTokenBody {
    pub id: Uuid,
}

impl AccessTokenBody for AdminAccessTokenBody {
    fn type_name() -> &'static str {
        "admin"
    }
}

impl Admin {
    pub fn publish_access_token(&self, pass: &str, exp: Duration) -> Result<AccessToken, Error> {
        self.cred.verify(pass).map_err(|_| Error::InvalidPassword)?;

        let body = AdminAccessTokenBody { id: self.id.0 };

        AccessToken::new(&body, exp).map_err(|_| Error::InvalidPassword)
    }
}
