pub mod errors;
pub mod logging;
mod ai_completions

use crate::logging::init_logging;
use errors::{ CustomError};

pub fn main() {
    init_logging().expect("Failed to initialize logger");
    
    log::info!("This is an info message SUCCESS");
}