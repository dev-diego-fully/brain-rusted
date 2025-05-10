use crate::lexing::Token;

/// Represents a brainfuck program instruction. This instruction (or a vector
/// of them) can be executed directly by the virtual machine.
#[derive(Debug, Clone)]
pub(crate) enum Instruction {
    Advance,
    Recede,
    Increment,
    Decrement,
    Show,
    Read,
    Loop(Vec<Instruction>),
}

impl Instruction {
    /// Returns an instruction that is equivalent to the given token. It
    /// should not be called with "Token::StartLoop" or "Token::StopLoop"
    /// as it does not return a suitable instruction in this scenario.
    pub(crate) fn from(token: &Token) -> Self {
        match token {
            Token::Advance => Self::Advance,
            Token::Decrement => Self::Decrement,
            Token::Increment => Self::Increment,
            Token::Read => Self::Read,
            Token::Recede => Self::Recede,
            Token::Show => Self::Show,
            Token::StartLoop => Self::Loop(Vec::new()),
            Token::StopLoop => Self::Loop(Vec::new()),
        }
    }

    /// Returns a loop instruction containing the given instruction vector
    /// in its body.
    pub(crate) fn loop_from(instructions: &[Instruction]) -> Self {
        Instruction::Loop(instructions.to_vec())
    }

    /// Returns the representation of a program in which there was an
    /// improper attempt to close a loop.
    pub(crate) fn represents_mismatched(instructions: &[Self]) -> String {
        Self::represents_loop(instructions)
            .chars()
            .skip(1)
            .collect()
    }

    /// Returns the representation of a loop that was not properly closed
    /// (left open). Receives only the loop instructions and not a
    /// loop itself.
    pub(crate) fn represents_unclosed(instructions: &[Self]) -> String {
        Self::represents_loop(instructions)
            .chars()
            .take(instructions.len() - 1)
            .collect()
    }
}

impl Instruction {
    fn representation(&self) -> String {
        match self {
            Instruction::Advance => String::from(">"),
            Instruction::Recede => String::from("<"),
            Instruction::Increment => String::from("+"),
            Instruction::Decrement => String::from("-"),
            Instruction::Show => String::from("."),
            Instruction::Read => String::from(","),
            Instruction::Loop(instructions) => Self::represents_loop(instructions),
        }
    }

    fn represents_loop(instructions: &[Self]) -> String {
        let ops_representation: Vec<String> =
            instructions.iter().map(|i| i.representation()).collect();
        format!("[{}]", ops_representation.join(""))
    }
}
