pub mod image;
pub mod shared;

use std::error::Error;
type TuiResult = Result<(), Box<dyn Error + Send + Sync>>;
