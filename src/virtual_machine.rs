use crate::lexer_parser::Instruction;
use std::io::Read;

/// Implements the virtual machine's public API.
impl VirtualMachine {
    /// Creates a new virtual machine completely empty. Its current memory
    /// slot will be the first one (0) and all slots will have 0 as their
    /// registered value.
    pub(crate) fn new() -> Self {
        Self {
            current_memory_slot: 0,
            memory_slots: vec![0; Self::MEMORY_SLOTS_COUNT],
        }
    }

    /// Returns a new virtual machine, but returns it after executing the
    /// instructions passed in the input vector.
    pub(crate) fn executing(instructions: &[Instruction]) -> Self {
        let mut vm = Self::new();
        vm.execute_instructions(instructions);
        vm
    }

    /// Executes each instruction of the given vector.
    pub(crate) fn execute_instructions(&mut self, program: &[Instruction]) {
        program
            .iter()
            .for_each(|instruction| self.execute_instruction(instruction));
    }
}

/// Brainf*ck Virtual Machine. It has 256 memory slots. Each slot stores one
/// byte and has no signal. The execution of the virtual machine will
/// never cause any type of error.
pub(crate) struct VirtualMachine {
    ///Virtual machine memory slots. Each slot can store a one-byte value.
    memory_slots: Vec<u8>,
    /// Index of the memory slot that is currently in use.
    /// This means that this is the slot where a value will be read or
    /// written if an instruction requests it.
    current_memory_slot: u8,
}

/// Implements commands for the virtual machine. A command is anything that
/// can be executed directly by an instruction, so everything here can be
/// executed directly by the user.
impl VirtualMachine {
    /// Executes the given instruction. Basically maps an instruction to a
    /// method of this VM.
    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Advance => self.move_to_next_slot(),
            Instruction::Recede => self.move_to_previous_slot(),
            Instruction::Increment => self.increment_slot_value(),
            Instruction::Decrement => self.decrement_slot_value(),
            Instruction::Show => self.display_from_current_memory_slot(),
            Instruction::Read => self.read_from_user(),
            Instruction::Loop(instructions) => self.execute_loop(instructions),
        }
    }

    /// If the value of the current slot is true, it executes all
    /// instructions in the input vector. After that, it repeat the entire
    /// operation since the check (as if in recursion).
    fn execute_loop(&mut self, instructions: &[Instruction]) {
        while self.check_current_memslot() {
            self.execute_instructions(instructions);
        }
    }

    /// Displays the value stored in the current memory slot as an ascii
    /// character on the terminal.
    fn display_from_current_memory_slot(&self) {
        print!("{}", char::from(self.current_memslot_value()));
    }

    /// Increments the value registered in the current memory slot. Does not
    /// prevent overflows.
    fn increment_slot_value(&mut self) {
        self.memory_slots[self.current_memory_slot as usize] += 1;
    }

    /// Decrements the value stored in the current memory slot. Does not
    /// prevent underflow.
    fn decrement_slot_value(&mut self) {
        self.memory_slots[self.current_memory_slot as usize] -= 1;
    }

    /// Advances to the next memory slot. Returns to the first slot if in
    /// the last memory slot.
    fn move_to_next_slot(&mut self) {
        self.current_memory_slot += 1;
    }

    /// Moves to the previous memory slot. Moves to the last slot if it
    /// is in the first slot.
    fn move_to_previous_slot(&mut self) {
        self.current_memory_slot -= 1;
    }

    /// Reads a character from the user and writes the value to the current
    /// memory slot. Supports only ascii characters
    fn read_from_user(&mut self) {
        let input: Option<u8> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok());

        let input: u8 = input.unwrap_or_default();

        self.memory_slots[self.current_memory_slot as usize] = input;
    }
}

/// Implements special virtual machine operations. Nothing here is executed
/// directly by an instruction and therefore cannot be executed directly by
/// the user.
impl VirtualMachine {
    /// Number of default memory slots for this virtual machine.
    const MEMORY_SLOTS_COUNT: usize = 256;

    /// Returns the value recorded in the current memory slot.
    fn current_memslot_value(&self) -> u8 {
        self.memory_slots[self.current_memory_slot as usize]
    }

    /// Returns whether or not the value recorded in the current memory
    /// slot is different from 0.
    fn check_current_memslot(&self) -> bool {
        self.current_memslot_value() != 0
    }
}
