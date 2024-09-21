#[cfg(test)]
mod tests {
    use super::*;  // Import the functions and types from the main module
    use syn_crabs::setup_logging;

    #[test]
    fn test_logger_initializes_successfully() {
        let log = setup_logging(false, false);
        // Call the setup_logging function to initialize the logger
        let result = setup_logging().expect("Failed to initialize logger");
        
        // Assert that logger setup was successful (it should not return an error)
        assert!(result.is_ok(), "Logger failed to initialize");

        // Optionally, you can log some messages to check logging functionality
        log::info!("Test info message");
        log::warn!("Test warning message");
        log::error!("Test error message");
    }
}
