use super::Instruction;
use crate::lexing::Token;

/// Type responsible for performing syntactic analysis. It can be
/// instantiated, but its public interface uses only its static method.
pub(crate) struct Parser {
    /// levels are used to analyze nested loops that are being constructed in
    /// the program. Level 0 is the level without loops, that is, the program
    /// itself, all other levels are loops.
    levels: Vec<Vec<Instruction>>,
}

/// Implements the public Parser API.
impl Parser {
    /// Attempts to convert the input token iterator vector into a vector
    /// of Instructions. If it fails, it returns an error message that can
    /// be displayed by the program.
    pub(crate) fn parse(instructions: &[Token]) -> Result<Vec<Instruction>, String> {
        let mut parser = Self::new();
        instructions
            .iter()
            .try_for_each(|tk| parser.parse_token(tk))
            .and_then(|_| parser.get_finished().cloned())
    }
}

/// Implements the private parser methods.
impl Parser {
    /// Attempts to return the current state of the parsing as a complete
    /// program. Will fail if there are unclosed loops. In case of failure
    /// returns the error message that can be displayed by the program.
    fn get_finished(&self) -> Result<&Vec<Instruction>, String> {
        if self.levels.len() > 1 {
            Err(super::error_unclosed_loop(self.levels.last().unwrap()))
        } else {
            Ok(&self.levels[0])
        }
    }

    /// Returns a new instance of Parser, ready to parse a program "from
    /// scratch". Its levels are defined as containing only level 0, and
    /// its level 0 will be empty.
    fn new() -> Self {
        Self {
            levels: vec![Vec::new()],
        }
    }

    /// Attempts to parse the given token, adding it to the appropriate level,
    /// creating a new level, or ending the current level. Whatever is
    /// appropriate for the input token. On failure, returns an error
    /// message that may be displayed by the program.
    fn parse_token(&mut self, token: &Token) -> Result<(), String> {
        match token {
            Token::StopLoop => self.stop_loop(),
            Token::StartLoop => {
                self.start_new_loop();
                Ok(())
            }
            tk => {
                let instruction = Instruction::from(tk);
                self.push_instruction(instruction);
                Ok(())
            }
        }
    }

    /// Starts a new loop, adding a new level to the levels vector.
    fn start_new_loop(&mut self) {
        self.levels.push(Vec::new());
    }

    /// Attempts to end the parsing of the current loop. Will fail if
    /// there is no loop being constructed at the moment (level with
    /// less than two elements). In case of failure, returns the error
    /// message that can be displayed by the program.
    fn stop_loop(&mut self) -> Result<(), String> {
        if self.levels.len() <= 1 {
            Err(super::error_mismatched_loop_closing(&self.levels[0]))
        } else {
            let popped = self.levels.pop().unwrap();
            let new_loop = Instruction::loop_from(&popped);
            self.push_instruction(new_loop);
            Ok(())
        }
    }

    /// Adds the given instruction to the instruction queue at the
    /// level currently being built.
    fn push_instruction(&mut self, instruction: Instruction) {
        if let Some(v) = self.levels.last_mut() {
            v.push(instruction);
        }
    }
}
