use anyhow::Result;
use log::error;
use ethical_hacking_lib::{simulate_file_error, simulate_network_error, simulate_permission_error, simulate_invalid_input}; // Assuming `lib.rs` is in `ethical_hacking_lib`



#[test]
fn test_file_not_found_error() {
    if let Err(e) = simulate_file_error() {
        error!("Test error: {:#?}", e);  // Ensure it logs the error
        assert!(e.to_string().contains("Failed to locate the config file"));
        assert!(e.to_string().contains("File not found"));
    }
}

#[test]
fn test_network_timeout_error() {
    if let Err(e) = simulate_network_error() {
        error!("Test error: {:#?}", e);  // Log error for debugging
        assert!(e.to_string().contains("Network request timed out"));
        assert!(e.to_string().contains("Network timeout"));
    }
}

#[test]
fn test_permission_denied_error() {
    if let Err(e) = simulate_permission_error() {
        error!("Test error: {:#?}", e);  // Log error for debugging
        assert!(e.to_string().contains("Access denied when trying to open /etc/passwd"));
        assert!(e.to_string().contains("Permission denied"));
    }
}

#[test]
fn test_invalid_input_error() {
    if let Err(e) = simulate_invalid_input() {
        error!("Test error: {:#?}", e);  // Log error for debugging
        assert!(e.to_string().contains("Failed to parse user input as a valid IP address"));
        assert!(e.to_string().contains("Invalid input detected"));
    }
}