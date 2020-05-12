use bytes::Bytes;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub name: String,
    pub data: Bytes,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("format {0:?} is not supported")]
    UnsupportedFormat(image::ImageFormat),
    #[error("image error")]
    ImageError(#[from] image::error::ImageError),
}

/*
 * =======
 * Command
 * =======
 */
impl Image {
    pub fn new(data: Bytes) -> Result<Self, Error> {
        let format_str = match image::guess_format(&data)? {
            image::ImageFormat::Png => "png",
            image::ImageFormat::Jpeg => "jpeg",
            f => return Err(Error::UnsupportedFormat(f)),
        };

        Ok(Image {
            name: format!("{}.{}", Uuid::new_v4(), format_str),
            data,
        })
    }
}
