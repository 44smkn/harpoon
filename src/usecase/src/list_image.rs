use domain::image::{ImageRepository, ImageSummary};
use std::error::Error;

pub struct ListImageUsecase<'a, T: ImageRepository> {
    repository: &'a T,
}

impl<'a, T: ImageRepository> ListImageUsecase<'a, T> {
    pub fn new(image_repository: &'a T) -> Self {
        Self {
            repository: image_repository,
        }
    }

    pub async fn list_image(&self) -> Result<Vec<ImageSummary>, Box<dyn Error + Send + Sync>> {
        let images = self.repository.list().await?;
        Ok(images)
    }
}
