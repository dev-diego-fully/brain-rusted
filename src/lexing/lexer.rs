use super::token::Token;

/// Static type that has lexing logic and serves as a public interface.
pub(crate) struct Lexer {}

impl Lexer {
    /// Attempts to return an array containing all tokens found during the
    /// given string. Ignores any whitespace found. Fails to find the first
    /// invalid character in the string. In this case, returns an error
    /// message indicating which character was found. This error message
    /// is suitable for display by the program.
    pub(crate) fn tokenize(program: &str) -> Result<Vec<Token>, String> {
        program
            .chars()
            .filter(|c| !Self::is_ignorable_token(c))
            .map(Token::from)
            .collect()
    }

    /// Returns whether or not a given character is ignorable during the
    /// tokenization process.
    fn is_ignorable_token(token: &char) -> bool {
        token.is_whitespace()
    }
}
