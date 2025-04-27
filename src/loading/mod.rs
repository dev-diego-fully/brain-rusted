//! This module encapsulates all the logic of loading the program's source
//! code from the file at the given path. It defines only a function that
//! attempts to return the entire contents of the file at once.

/// This module defines a function to facilitate the generation of error
/// messages that occur during the program file loading process. These
/// error messages are suitable for display by the program.
mod errors;
/// This module defines functions to load a file from a command line
/// argument.
pub(crate) mod load;

use errors::{error_file_not_found, error_reading_file};
pub(crate) use load::load_program_file;
