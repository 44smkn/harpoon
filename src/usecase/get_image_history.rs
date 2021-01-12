use crate::domain::image::{ImageHistory, ImageRepository};
use std::error::Error;

pub struct GetImageHistoryUsecase<'a, T: ImageRepository> {
    repository: &'a T,
}

impl<'a, T: ImageRepository> GetImageHistoryUsecase<'a, T> {
    pub fn new(image_repository: &'a T) -> Self {
        GetImageHistoryUsecase {
            repository: image_repository,
        }
    }

    pub async fn get_history(
        &self,
        id: impl Into<String>,
    ) -> Result<ImageHistory, Box<dyn Error + Send + Sync>> {
        let history = self.repository.history(id.into()).await?;
        Ok(history)
    }
}
