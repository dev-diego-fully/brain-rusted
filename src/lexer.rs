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
            .filter(|c| !Self::is_ignorable_token(*c))
            .map(Token::from)
            .collect()
    }

    /// Returns whether or not a given character is ignorable during the
    /// tokenization process.
    fn is_ignorable_token(token: char) -> bool {
        token.is_whitespace()
    }
}

/// Type represents each of the tokens that can be present in a
/// brainf*ck program
#[derive(Debug)]
pub(crate) enum Token {
    Advance,
    Decrement,
    Increment,
    Read,
    Recede,
    Show,
    StartLoop,
    StopLoop,
}

impl Token {
    /// Attempts to return the token that is equivalent to the given
    /// characters. If it fails, it returns an error message that informs
    /// which invalid character was found. This error message can
    /// be displayed by the program.
    fn from(token: char) -> Result<Token, String> {
        match token {
            '>' => Ok(Token::Advance),
            '<' => Ok(Token::Recede),
            '+' => Ok(Token::Increment),
            '-' => Ok(Token::Decrement),
            '.' => Ok(Token::Show),
            '[' => Ok(Token::StartLoop),
            ']' => Ok(Token::StopLoop),
            ',' => Ok(Token::Read),
            _ => Err(error_invalid_token_found(token)),
        }
    }
}

/// Returns an error message when an invalid character is encountered during
/// the tokenization process.
fn error_invalid_token_found(token: char) -> String {
    format!("Invalid token found: {}", token)
}
