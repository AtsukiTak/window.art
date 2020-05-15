use chrono::{DateTime, Duration, Utc};
use serde::{de::DeserializeOwned, ser::Serialize};

pub struct AccessToken<B> {
    pub body: B,
    pub expire_at: DateTime<Utc>,
}

impl<B> AccessToken<B>
where
    B: Serialize + DeserializeOwned,
{
    pub fn new(body: B, valid_dur: Duration) -> AccessToken<B> {
        AccessToken {
            body,
            expire_at: Utc::now() + valid_dur,
        }
    }
}

pub trait AccessTokenEncoder {
    fn encode<B>(&self, token: AccessToken<B>) -> anyhow::Result<String>
    where
        B: Serialize;

    fn decode<B>(&self, s: &str) -> anyhow::Result<AccessToken<B>>
    where
        B: DeserializeOwned;
}
