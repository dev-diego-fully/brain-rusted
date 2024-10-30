mod lexer_parser;
mod virtual_machine;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::ErrorKind;

use virtual_machine::VirtualMachine;
use lexer_parser::{
    Token,
    Instruction
};


fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let path: &String;

    if args.len() > 1 {

        path = &args[ 1 ];

        let file = File::open( path )?;
        let mut buf_reader: BufReader<File> = BufReader::new( file );
        let mut contents: String = String::new();

        buf_reader.read_to_string( &mut contents )?;

        let mut vm = VirtualMachine::new();
        vm.exec_program(
            Instruction::from(
                &Token::parse(
                    contents
                )
            )
        );

        return Ok( () );

    } else {

        println!( "Error: No path passed.\n" );

        return Err( std::io::Error::from( ErrorKind::InvalidInput ) );

    }

}
