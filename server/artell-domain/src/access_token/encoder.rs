use super::AccessToken;
use serde::{de::DeserializeOwned, ser::Serialize};

pub trait AccessTokenEncoder {
    type Error;

    fn encode<B>(&self, token: AccessToken<B>) -> Result<String, Self::Error>
    where
        B: Serialize;

    fn decode<B>(&self, s: &str) -> Result<AccessToken<B>, Self::Error>
    where
        B: DeserializeOwned;
}
