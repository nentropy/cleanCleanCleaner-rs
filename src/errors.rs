use anyhow::{Result, Context};
use log::{info, warn, error};
use thiserror::Error;
use crate::logging::init_logging;


// Custom error types using `thiserror`.
#[derive(Debug, Error)]
pub enum CustomError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Network timeout: {0}")]
    NetworkTimeout(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Port {0} is already in use")]
    PortInUse(u16),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

// Convert common error types into `CustomError`.
impl From<std::io::Error> for CustomError {
    fn from(err: std::io::Error) -> Self {
        CustomError::UnexpectedError(err.to_string())
    }
}


// Example function that simulates an error (file not found).
pub fn simulate_file_error() -> Result<()> {
    init_logging().expect("Failed to initialize logger");
    log::info!("Starting file operation");
    Err(CustomError::FileNotFound("config.txt".into())).context("Config file not found")
}

// Example function that simulates a network timeout error.
pub fn simulate_network_error() -> Result<()> {
    log::info!("Attempting to connect to the server");
    Err(CustomError::NetworkTimeout("Server unreachable".into())).context("Network request timed out")
}

// Function to handle errors and log them based on type.
pub fn handle_error(result: Result<()>) {
    if let Err(err) = result {
        if let Some(custom_err) = err.downcast_ref::<CustomError>() {
            // Match on the specific custom error type for logging.
            match custom_err {
                CustomError::FileNotFound(file) => error!("File not found: {file}"),
                CustomError::NetworkTimeout(reason) => warn!("Network timeout: {reason}"),
                CustomError::InvalidInput(input) => info!("Invalid input: {input}"),
                CustomError::PermissionDenied(resource) => error!("Permission denied: {resource}"),
                CustomError::PortInUse(port) => warn!("Port {port} is already in use"),
                CustomError::UnexpectedError(reason) => error!("Unexpected error: {reason}"),
            }
        } else {
            log::error!("An unexpected error occurred: {:#?}", err);
        }
    }
}

// Main function to demonstrate handling different errors.
fn main() -> anyhow::Result<()> {
    init_logging().expect("Failed to initialize logger");
    handle_error(simulate_file_error());
    handle_error(simulate_network_error());

    Ok(())
}
