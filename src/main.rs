mod lexing;
mod loading;
mod parsing;
mod virtual_machine;

use lexing::Lexer;
use parsing::Parser;
use virtual_machine::VirtualMachine;

fn main() {
    let _ = loading::load_program_file()
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
