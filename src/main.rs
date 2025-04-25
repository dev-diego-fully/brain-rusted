mod lexer;
mod load;
mod parser;
mod virtual_machine;

use lexer::Lexer;
use parser::Parser;
use virtual_machine::VirtualMachine;

fn main() {
    let _ = load::load_program_file()
        .and_then(|content| Lexer::tokenize(&content))
        .and_then(|tokens| Parser::parse(&tokens))
        .map(|instructions| {
            VirtualMachine::executing(&instructions);
        })
        .or_else(|msg| {
            println!("{}", msg);
            Ok::<(), String>(())
        });
}
