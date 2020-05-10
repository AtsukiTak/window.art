use artell_domain::image::{Image, ImageRepository};
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
    fn url_to(&self, image: &Image) -> String {
        self.path
            .with_file_name(image.name())
            .into_os_string()
            .into_string()
            .unwrap()
    }

    async fn save(&self, image: Image) -> anyhow::Result<()> {
        let path = self.url_to(&image);
        std::fs::write(path, image.data)?;
        Ok(())
    }
}
