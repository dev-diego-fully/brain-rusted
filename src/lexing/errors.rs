/// Returns an error message when an invalid character is encountered during
/// the tokenization process.
pub(crate) fn error_invalid_token_found(token: char) -> String {
    format!("Invalid token found: {}", token)
}
