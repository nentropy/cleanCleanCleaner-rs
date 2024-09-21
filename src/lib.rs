pub mod errors;
pub mod logging;
mod ai_completions;

use logging::init_logging;

pub fn main() {
let _log = init_logging().expect("Failed to initialize logger");
log::info!("Logging init..")

}