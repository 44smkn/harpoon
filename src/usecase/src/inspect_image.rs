use domain::image::ImageRepository;
use std::collections::HashMap;
use std::error::Error;

pub struct InspectImageUsecase<'a, T: ImageRepository> {
    repository: &'a T,
}

impl<'a, T: ImageRepository> InspectImageUsecase<'a, T> {
    pub fn new(image_repository: &'a T) -> Self {
        Self {
            repository: image_repository,
        }
    }

    pub async fn inspect_image(&self, id: impl Into<String>) -> Result<InspectImageDto, Box<dyn Error + Send + Sync>> {
        let id = id.into();
        let detail = self.repository.inspect(id.clone());
        let history = self.repository.history(id);

        let detail = detail.await?;
        let history = history.await?;
        let dto = InspectImageDto {
            id: detail.id,
            os: detail.os,
            architecture: detail.architecture,
            entrypoint: detail.entrypoint,
            cmd: detail.cmd,
            environment_variables: detail.env,
            labels: detail.labels,
            history: history
                .into_iter()
                .map(|v| HistoryRecord {
                    image_id: v.id,
                    created_by: v.created_by,
                    size: v.size,
                })
                .collect(),
        };
        Ok(dto)
    }
}

pub struct InspectImageDto {
    pub id: String,
    pub os: String,
    pub architecture: String,
    pub entrypoint: Vec<String>,
    pub cmd: Vec<String>,
    pub environment_variables: Vec<String>,
    pub labels: HashMap<String, String>,
    pub history: Vec<HistoryRecord>,
}

pub struct HistoryRecord {
    pub image_id: String,
    pub created_by: String,
    pub size: i32,
}
