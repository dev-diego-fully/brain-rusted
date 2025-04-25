use std::io::Read;

/// Attempts to return, as a string, all the content present in the file at
/// the path given by the program's first argument. In case of failure,
/// returns an error message explaining the problem encountered. This
/// message is appropriate for display by the program.
pub(crate) fn load_program_file() -> Result<String, String> {
    get_program_path()
        .and_then(|path: std::string::String| open_file(&path))
        .and_then(|file: std::fs::File| get_file_content(&file))
}

/// On failure, return an error message saying that a path was not provided.
/// This error message is suitable for display by the program.
fn get_program_path() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        return Err("No file found".to_string());
    }

    Ok(args[1].clone())
}

/// Attempts to open and return the file at the given path. If this fails, a
/// message is returned explaining that a file could not be found at the
/// given path. This message is suitable for display by the program.
fn open_file(path: &String) -> Result<std::fs::File, String> {
    std::fs::File::open(path).map_err(|_| error_file_not_found(path))
}

/// Attempts to return, as a string, the entire contents of the given file.
/// If this fails, it returns an error message stating that it was not
/// possible to read the contents of the given file. This error message
/// is suitable for display by the program.
fn get_file_content(file: &std::fs::File) -> Result<String, String> {
    let mut buff_reader = std::io::BufReader::new(file);
    let mut content = String::new();

    match buff_reader.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(_) => Err(error_reading_file()),
    }
}

/// Returns an error message when a file cannot be found with the specified
/// path.
fn error_file_not_found(path: &String) -> String {
    format!("File not found: {}", path)
}

/// Returns an error message when it was not possible to load the
/// contents of the file in the program argument path, for a reason
/// other than finding it.
fn error_reading_file() -> String {
    "Failed to read file content".to_string()
}
