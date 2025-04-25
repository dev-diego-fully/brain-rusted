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
    pub fn from(token: String) -> Option<Token> {
        match token.as_str() {
            ">" => Some(Token::Advance),
            "<" => Some(Token::Recede),
            "+" => Some(Token::Increment),
            "-" => Some(Token::Decrement),
            "." => Some(Token::Show),
            "[" => Some(Token::StartLoop),
            "]" => Some(Token::StopLoop),
            "," => Some(Token::Read),
            _ => None,
        }
    }

    pub fn parse(program: String) -> Vec<Token> {
        let mut parsed: Vec<Token> = Vec::new();

        for c in program
            .replace(" ", "")
            .replace("\t", "")
            .replace("\n", "")
            .chars()
        {
            let tk = String::from(c);

            match Self::from(tk) {
                Some(v) => parsed.push(v),
                None => panic!("Invalid token gotted."),
            }
        }

        parsed
    }
}

#[derive(Debug)]
pub enum Instruction {
    Advance,
    Recede,
    Increment,
    Decrement,
    Show,
    Read,
    Loop(Vec<Instruction>),
}

impl Instruction {
    pub fn from(program: &Vec<Token>) -> Vec<Instruction> {
        let (program, _len) = Self::priv_from(program, 0, 0);

        program
    }

    fn priv_from(program: &Vec<Token>, i: usize, loop_level: u8) -> (Vec<Instruction>, usize) {
        let mut parsed: Vec<Instruction> = Vec::new();
        let mut j: usize = i;
        let mut len: usize = 0;

        while j < program.len() {
            parsed.push(match program[j] {
                Token::Advance => {
                    len += 1;
                    j += 1;
                    Instruction::Advance
                }
                Token::Recede => {
                    len += 1;
                    j += 1;
                    Instruction::Recede
                }
                Token::Increment => {
                    len += 1;
                    j += 1;
                    Instruction::Increment
                }
                Token::Decrement => {
                    len += 1;
                    j += 1;
                    Instruction::Decrement
                }
                Token::Show => {
                    len += 1;
                    j += 1;
                    Instruction::Show
                }
                Token::StartLoop => {
                    let (result, r_len) = Self::priv_from(program, j + 1, loop_level + 1);
                    len += r_len;
                    j += r_len;
                    Instruction::Loop(result)
                }
                Token::Read => {
                    len += 1;
                    j += 1;
                    Instruction::Read
                }
                Token::StopLoop => {
                    len += 2;
                    if loop_level > 0 {
                        return (parsed, len);
                    } else {
                        panic!("Invalid loop level");
                    }
                }
            });
        }

        (parsed, len)
    }
}
