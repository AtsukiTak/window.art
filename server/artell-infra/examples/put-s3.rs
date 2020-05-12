use artell_domain::image::{Image, ImageRepository};
use artell_infra::s3::S3ImageRepository;
use bytes::Bytes;

#[tokio::main]
async fn main() {
    let image = Image::new(Bytes::from(
        std::fs::read("/Users/takahashiatsuki/Desktop/myself.jpg").unwrap(),
    ))
    .unwrap();
    S3ImageRepository::new("artell".to_string())
        .save(image)
        .await
        .unwrap()
}
