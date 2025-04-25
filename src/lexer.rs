#[derive(Debug)]
pub enum Token {
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
    fn from(token: char) -> Option<Token> {
        match token {
            '>' => Some(Token::Advance),
            '<' => Some(Token::Recede),
            '+' => Some(Token::Increment),
            '-' => Some(Token::Decrement),
            '.' => Some(Token::Show),
            '[' => Some(Token::StartLoop),
            ']' => Some(Token::StopLoop),
            ',' => Some(Token::Read),
            _ => None,
        }
    }

    pub fn parse(program: String) -> Vec<Token> {
        let mut parsed: Vec<Token> = Vec::new();

        for c in program.chars().filter(|c| !Self::is_ignorable_token(*c)) {
            match Self::from(c) {
                Some(v) => parsed.push(v),
                None => panic!("Invalid token gotted."),
            }
        }

        parsed
    }

    fn is_ignorable_token(token: char) -> bool {
        token.is_whitespace()
    }
}
