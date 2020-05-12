use artell_domain::image::{Image, ImageRepository};
use futures::{future, stream};
use rusoto_core::{region::Region, ByteStream};
use rusoto_s3::{PutObjectRequest, S3Client, S3};

pub struct S3ImageRepository {
    client: S3Client,
    bucket: String,
}

impl S3ImageRepository {
    pub fn new(bucket: String) -> Self {
        let client = S3Client::new(Region::ApNortheast1);
        S3ImageRepository { client, bucket }
    }
}

#[async_trait]
impl ImageRepository for S3ImageRepository {
    fn url_to(&self, image_name: &str) -> String {
        format!(
            "https://artell.s3-ap-northeast-1.amazonaws.com/{}",
            image_name
        )
    }

    async fn save(&self, image: Image) -> anyhow::Result<()> {
        log::debug!("start putting a new object to s3...");
        self.client
            .put_object(PutObjectRequest {
                bucket: self.bucket.clone(),
                key: image.name,
                body: Some(ByteStream::new(stream::once(future::ok(image.data)))),
                ..Default::default()
            })
            .await?;
        log::debug!("completed to put a new object to s3");
        Ok(())
    }
}
