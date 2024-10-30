use crate::lexer_parser::Instruction;
use std::io::Read;


//attributes
pub struct VirtualMachine {

    current: u8,
    states: Vec<u8>

}

//instances
impl VirtualMachine {

    pub fn increment( &mut self ) {

        self.states[ self.current as usize ] += 1;

    }

    pub fn decrement( &mut self ) {

        self.states[ self.current as usize ] -= 1;

    }

    pub fn advance( &mut self ) {

        self.current += 1;

    }

    pub fn recede( &mut self ) {

        self.current -= 1;

    }

    pub fn read( &mut self ) {

        let input: Option<u8> = std::io::stdin().bytes()
                                                .next()
                                                .and_then(
                                                    |result| result.ok()
                                                )
                                                .map( |byte| byte as u8 );

        let input: u8 = match input {
            Some( v ) => v,
            None => 0
        };

        self.states[ self.current as usize ] = input;

    }

    pub fn show( &mut self ) {

        print!( "{}", char::from( self.states[ self.current as usize ] as u8 ) );

    }

    pub fn check( &mut self ) -> bool {

        return self.states[ self.current as usize ] != 0;

    }

    pub fn exec( &mut self, instruction: &Instruction ) {

        match instruction {
            Instruction::Advance => self.advance(),
            Instruction::Recede => self.recede(),
            Instruction::Increment => self.increment(),
            Instruction::Decrement => self.decrement(),
            Instruction::Show => self.show(),
            Instruction::Read => self.read(),
            Instruction::Loop( instructions ) => {
                while self.check() {
                    for instruction in instructions {
                        self.exec( &instruction );
                    }
                }
            }
        }

    }

    pub fn exec_program( &mut self, program: Vec<Instruction> ) {

        for i in program {
            self.exec( &i );
        }

    }

}

//statics
impl VirtualMachine {

    pub fn new() -> Self {

        let a = Self {
            current: 0,
            states: vec![0; 256]
        };

        return a;

    }

}
