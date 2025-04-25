use crate::lexer::Token;

pub(crate) struct Parser {}

impl Parser {
    pub(crate) fn parse(instructions: &[Token]) -> Result<Vec<Instruction>, String> {
        Ok(Instruction::from(instructions))
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
    pub fn from(program: &[Token]) -> Vec<Instruction> {
        let (program, _len) = Self::priv_from(program, 0, 0);

        program
    }

    fn priv_from(program: &[Token], i: usize, loop_level: u8) -> (Vec<Instruction>, usize) {
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
