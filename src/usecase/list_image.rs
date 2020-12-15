use crate::domain::image::ImageRepository;
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

    pub async fn list_image(&self) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>> {
        let images = self.repository.list().await?;
        let mut items: Vec<Vec<String>> = Vec::new();

        for mut image in images.into_iter() {
            if &image.repo_tags[0] == "<none>:<none>" {
                continue;
            }
            let mut row: Vec<String> = Vec::new();
            row.push(std::mem::take(&mut image.repo_tags[0]));
            let size = f64::from(image.size) / 1000000.0;
            row.push(format!("{:.2}MB", size));
            row.push(image.created.format("%Y-%m-%d %H:%M:%S").to_string());
            items.push(row);
        }
        Ok(items)
    }
}