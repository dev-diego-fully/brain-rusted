//! The parsing module encapsulates the program's syntactic analysis (or
//! parsing) logic. This analysis must be executed after the lexical
//! analysis. Its result can be executed directly by the virtual machine.

/// This module defines functions to facilitate the generation of error
/// messages that occur during the parsing process. These error messages
/// are suitable for display by the program.
mod errors;

/// This module defines the Instruction type. This type can be executed
/// properly by the virtual machine.
pub(crate) mod instruction;

/// This module defines the Parser type, which interfaces with the program's
/// parser and allows transforming an input of tokens into a set of
/// instructions that can be executed by the virtual machine.
pub(crate) mod parser;

use errors::{error_mismatched_loop_closing, error_unclosed_loop};
pub(crate) use instruction::Instruction;
pub(crate) use parser::Parser;
