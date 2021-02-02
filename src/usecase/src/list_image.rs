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

#[cfg(test)]
mod tests {
    use infrastructure::webapi::inmem::image_repository::FakeImageRepository;
    use tokio;
    use usecase::list_image::ListImageUsecase;

    #[tokio::test]
    async fn listing() {
        let image_repository = FakeImageRepository::new();
        let list_image_usecase = ListImageUsecase::new(&image_repository);
        let image_summaries = list_image_usecase.list_image().await;
        assert!(image_summaries.is_ok());
    }
}
