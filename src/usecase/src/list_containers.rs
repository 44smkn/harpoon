use domain::container::{ContainerRepository, ContainerSummary};
use std::error::Error;

pub struct ListContainerUsecase<'a> {
    repository: &'a dyn ContainerRepository,
}

impl<'a> ListContainerUsecase<'a> {
    pub fn new(container_repository: &'a dyn ContainerRepository) -> Self {
        Self {
            repository: container_repository,
        }
    }

    pub async fn list_container(&self) -> Result<Vec<ContainerSummary>, Box<dyn Error + Send + Sync>> {
        let containers = self.repository.list().await?;
        Ok(containers)
    }
}
