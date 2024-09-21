use syn_crabs::setup_logging;

/// Initialize the logger. This should be called once during program startup.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging(false, false)?;

    Ok(())
}

