use bytes::Bytes;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub id: ImageId,
    pub data: Bytes,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageId(pub Uuid);

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    #[error("invalid data")]
    InvalidData,
}

/*
 * ========
 * Query
 * ========
 */
impl Image {
    pub fn path(&self) -> String {
        format!("{}.png", self.id.0)
    }
}

/*
 * =======
 * Command
 * =======
 */
impl Image {
    pub fn new(data: Bytes) -> Result<Self, Error> {
        // TODO
        // validate data

        Ok(Image {
            id: ImageId(Uuid::new_v4()),
            data,
        })
    }
}
