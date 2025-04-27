//! The lexing module encapsulates all the lexical analysis logic of the
//! program. It defines the Lexer (which serves as an interface for lexical
//! analysis) and Token (important for the next step, which is parsing) types.

/// This module defines the Lexer type, which is responsible for transforming
/// a text input into a set of tokens, which can be more easily converted
/// into virtual machine instructions.
pub(crate) mod lexer;

/// This module defines the Token type, which can be easily obtained from a
/// text character.
pub(crate) mod token;

/// This module makes it easy to create error messages for problems that
/// occur during the lexing process. These error messages are suitable
/// for display by the program.
mod errors;

use errors::error_invalid_token_found;
pub(crate) use lexer::Lexer;
pub(crate) use token::Token;
