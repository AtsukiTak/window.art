use bytes::Bytes;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub id: ImageId,
    pub data: Bytes,
    pub format: ImageFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImageId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("format {0:?} is not supported")]
    UnsupportedFormat(image::ImageFormat),
    #[error("image error")]
    ImageError(#[from] image::error::ImageError),
}

/*
 * ========
 * Query
 * ========
 */
impl Image {
    pub fn name(&self) -> String {
        let extension = match self.format {
            ImageFormat::Png => "png",
        };

        format!("{}.{}", self.id.0, extension)
    }
}

/*
 * =======
 * Command
 * =======
 */
impl Image {
    pub fn new(data: Bytes) -> Result<Self, Error> {
        let format = match image::guess_format(&data)? {
            image::ImageFormat::Png => ImageFormat::Png,
            f => return Err(Error::UnsupportedFormat(f)),
        };

        Ok(Image {
            id: ImageId(Uuid::new_v4()),
            data,
            format,
        })
    }
}
