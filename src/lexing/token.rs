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
    pub(crate) fn from(token: char) -> Result<Token, String> {
        match token {
            '>' => Ok(Token::Advance),
            '<' => Ok(Token::Recede),
            '+' => Ok(Token::Increment),
            '-' => Ok(Token::Decrement),
            '.' => Ok(Token::Show),
            '[' => Ok(Token::StartLoop),
            ']' => Ok(Token::StopLoop),
            ',' => Ok(Token::Read),
            _ => Err(super::error_invalid_token_found(token)),
        }
    }
}
