use serde::Serialize;
use std::fs;
use std::path::Path;
use chrono::Local;
use anyhow::{Result, Context};

/// Saves a vector of serializable data to a JSON file.
///
/// # Arguments
///
///  `data` - A vector of data that implements the Serialize trait
///  `file_name` - The name of the file to save (without extension)
///
/// # Returns
///
///  `Result<(), anyhow::Error>` - Ok(()) if successful, or an error if something went wrong
///
/// # Examples
///
/// ```
/// use your_crate::utils::save_to_json;
///
/// #[derive(Serialize)]
/// struct TestData {
///     id: i32,
///     name: String,
/// }
///
/// let data = vec![
///     TestData { id: 1, name: "Alice".to_string() },
///     TestData { id: 2, name: "Bob".to_string() },
/// ];
///
/// save_to_json(&data, "test_data").unwrap();
/// ```
/// 
pub fn save_to_json<T: Serialize>(data: &[T], file_name: &str) -> Result<()> {
    // Create the directory if it doesn't exist
    let dir_path = Path::new("./safe/tmp/reports/json");
    fs::create_dir_all(dir_path)
        .with_context(|| format!("Failed to create directory: {}", dir_path.display()))?;

    // Generate a timestamp for the file name
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("{}_{}.json", file_name, timestamp);

    // Construct the full file path
    let file_path = dir_path.join(file_name);

    // Serialize the data to JSON
    let json = serde_json::to_string_pretty(data)
        .with_context(|| "Failed to serialize data to JSON")?;

    // Write the JSON to the file
    fs::write(&file_path, json)
        .with_context(|| format!("Failed to write JSON to file: {}", file_path.display()))?;

    println!("Data saved to: {}", file_path.display());

    Ok(())
}