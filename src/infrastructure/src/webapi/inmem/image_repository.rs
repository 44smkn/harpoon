use async_trait::async_trait;
use domain::image::{Image, ImageHistory, ImageRepository, ImageSummary};
use std::error::Error;

pub struct FakeImageRepository {}

impl FakeImageRepository {
    pub fn new() -> Self {
        FakeImageRepository {}
    }
}

#[async_trait]
impl ImageRepository for FakeImageRepository {
    async fn list(&self) -> Result<Vec<ImageSummary>, Box<dyn Error + Send + Sync>> {
        unimplemented!();
    }
    #[allow(unused_variables)]
    async fn inspect(&self, id: String) -> Result<Image, Box<dyn Error + Send + Sync>> {
        unimplemented!();
    }
    #[allow(unused_variables)]
    async fn history(&self, id: String) -> Result<ImageHistory, Box<dyn Error + Send + Sync>> {
        unimplemented!();
    }
}
