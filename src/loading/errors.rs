/// Returns an error message when a file cannot be found with the specified
/// path.
pub(crate) fn error_file_not_found(path: &String) -> String {
    format!("File not found: {}", path)
}

/// Returns an error message when it was not possible to load the
/// contents of the file in the program argument path, for a reason
/// other than finding it.
pub(crate) fn error_reading_file() -> String {
    "Failed to read file content".to_string()
}
