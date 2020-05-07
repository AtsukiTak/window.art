use artell_domain::image::{Image, ImageId, ImageRepository};
use std::path::PathBuf;

pub struct FsImageRepository {
    path: PathBuf,
}

impl FsImageRepository {
    pub fn new(path: PathBuf) -> Self {
        assert!(path.is_dir());

        FsImageRepository { path }
    }
}

#[async_trait]
impl ImageRepository for FsImageRepository {
    fn path_to(&self, image_id: &ImageId) -> PathBuf {
        self.path.with_file_name(format!("{}.png", image_id.0))
    }

    async fn save(&self, image: Image) -> anyhow::Result<()> {
        let path = self.path_to(&image.id);
        std::fs::write(path, image.data)?;
        Ok(())
    }
}
