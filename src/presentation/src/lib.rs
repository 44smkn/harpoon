pub mod image;
pub mod shared;

use std::error::Error;
type TuiResult = Result<TuiOperationStatus, Box<dyn Error + Send + Sync>>;

#[derive(PartialOrd, PartialEq, Debug)]
pub enum TuiOperationStatus {
    Continue,
    Quit,
}
