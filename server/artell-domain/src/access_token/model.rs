use chrono::{DateTime, Duration, Utc};
use serde::{de::DeserializeOwned, ser::Serialize};

pub struct AccessToken<B> {
    pub body: B,
    pub expire_at: Option<DateTime<Utc>>,
}

impl<B> AccessToken<B>
where
    B: Serialize + DeserializeOwned,
{
    pub fn with_exp(body: B, valid_dur: Duration) -> AccessToken<B> {
        AccessToken {
            body,
            expire_at: Some(Utc::now() + valid_dur),
        }
    }

    pub fn without_exp(body: B) -> AccessToken<B> {
        AccessToken {
            body,
            expire_at: None,
        }
    }
}
