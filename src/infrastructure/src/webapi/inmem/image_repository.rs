use domain::image::{Image, ImageHistory, ImageRecord, ImageRepository, ImageSummary};

pub struct FakeImageRepository {}

impl ImageRepository for FakeImageRepository {
    async fn list(&self) -> Result<Vec<ImageSummary>, Box<dyn Error + Send + Sync>> {
        unimplemented!();
    }
    async fn inspect(&self, id: String) -> Result<Image, Box<dyn Error + Send + Sync>> {
        unimplemented!();
    }
    async fn history(&self, id: String) -> Result<ImageHistory, Box<dyn Error + Send + Sync>> {
        unimplemented!();
    }
}
