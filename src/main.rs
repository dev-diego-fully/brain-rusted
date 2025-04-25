mod lexer;
mod parser;
mod virtual_machine;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::ErrorKind;
use std::io::Read;

use lexer::Token;
use parser::Instruction;
use virtual_machine::VirtualMachine;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path: &String;

    if args.len() > 1 {
        path = &args[1];

        let file = File::open(path)?;
        let mut buf_reader: BufReader<File> = BufReader::new(file);
        let mut contents: String = String::new();

        buf_reader.read_to_string(&mut contents)?;
        let program = &Instruction::from(&Token::parse(contents));

        VirtualMachine::executing(program);

        Ok(())
    } else {
        println!("Error: No path passed.\n");

        Err(std::io::Error::from(ErrorKind::InvalidInput))
    }
}
