use domain::image::{ImageRepository, ImageSummary};
use std::error::Error;

pub struct ListImageUsecase<'a> {
    repository: &'a dyn ImageRepository,
}

impl<'a> ListImageUsecase<'a> {
    pub fn new(image_repository: &'a dyn ImageRepository) -> Self {
        Self {
            repository: image_repository,
        }
    }

    pub async fn list_image(&self) -> Result<Vec<ImageSummary>, Box<dyn Error + Send + Sync>> {
        let images = self.repository.list().await?;
        Ok(images)
    }
}
