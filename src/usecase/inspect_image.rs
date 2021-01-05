use crate::domain::image::{ImageDetail, ImageRepository};
use std::error::Error;

pub struct InspectImageUsecase<T: ImageRepository> {
    repository: T,
}

impl<T: ImageRepository> InspectImageUsecase<T> {
    pub fn new(image_repository: T) -> Self {
        InspectImageUsecase {
            repository: image_repository,
        }
    }

    pub async fn inspect_image(
        &self,
        id: impl Into<String>,
    ) -> Result<ImageDetail, Box<dyn Error + Send + Sync>> {
        let detail = self.repository.inspect(id.into()).await?;
        Ok(detail)
    }
}
