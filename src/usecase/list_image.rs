use crate::domain::image::{Image, ImageRepository};
use std::error::Error;

pub struct ListImageUsecase<T: ImageRepository> {
    repository: T,
}

impl<T: ImageRepository> ListImageUsecase<T> {
    pub fn new(image_repository: T) -> Self {
        ListImageUsecase {
            repository: image_repository,
        }
    }

    pub async fn list_image(&self) -> Result<Vec<Image>, Box<dyn Error + Send + Sync>> {
        let images = self.repository.list().await?;
        Ok(images)
    }
}
