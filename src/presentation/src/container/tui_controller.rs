use crate::TuiResult;
use usecase::list_containers::ListContainerUsecase;

pub struct ContainerTuiController<'a> {
    list_usecase: &'a ListContainerUsecase<'a>,
}

impl<'a> ContainerTuiController {
    pub fn new(list_usecase: &'a ListContainerUsecase<'a>) -> Self {
        Self { list_usecase }
    }

    pub async fn draw(&self, terminal: &mut Terminal<impl Backend>, events: &Events) -> TuiResult {
        unimplemented!();
    }
}
