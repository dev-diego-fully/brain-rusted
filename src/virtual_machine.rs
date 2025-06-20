use std::io::Read;

use crate::parsing::Instruction;

/// Implements the virtual machine's public API.
impl VirtualMachine {
    /// Creates a new virtual machine completely empty. Its current memory
    /// slot will be the first one (0) and all slots will have 0 as their
    /// registered value.
    pub(crate) fn new() -> Self {
        Self {
            current_memslot: 0,
            memory_slots: vec![Self::MEMSLOTS_INITIAL_VALUE; Self::MEMSLOTS_COUNT],
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
    current_memslot: u8,
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
            Instruction::Show => self.display_from_current_memslot(),
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
    fn display_from_current_memslot(&self) {
        print!("{}", char::from(self.get_current_memslot_value()));
    }

    /// Increments the value registered in the memory slot currently in use.
    /// In case of overflow, the value wraps
    fn increment_slot_value(&mut self) {
        let new_value = self.get_current_memslot_value().wrapping_add(1);
        self.set_current_memslot_value(new_value);
    }

    /// Decrements the value registered in the memory slot currently in use.
    /// In case of underflow, the value wraps
    fn decrement_slot_value(&mut self) {
        let new_value = self.get_current_memslot_value().wrapping_sub(1);
        self.set_current_memslot_value(new_value);
    }

    /// Advances to the next memory slot. Returns to the first slot if in
    /// the last memory slot.
    fn move_to_next_slot(&mut self) {
        self.current_memslot = self.current_memslot.wrapping_add(1);
    }

    /// Moves to the previous memory slot. Moves to the last slot if it
    /// is in the first slot.
    fn move_to_previous_slot(&mut self) {
        self.current_memslot = self.current_memslot.wrapping_sub(1);
    }

    /// Reads a character from the user and writes the value to the current
    /// memory slot. Supports only ascii characters
    fn read_from_user(&mut self) {
        let input: Option<u8> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok());

        let input: u8 = input.unwrap_or_default();
        self.set_current_memslot_value(input);
    }
}

/// Implements special virtual machine operations. Nothing here is executed
/// directly by an instruction and therefore cannot be executed directly by
/// the user.
impl VirtualMachine {
    /// Number of default memory slots for this virtual machine.
    const MEMSLOTS_COUNT: usize = 256;

    /// Default value written to a memory slot when the virtual machine
    /// is created.
    const MEMSLOTS_INITIAL_VALUE: u8 = 0;

    /// Value treated as false in BF. Everything else is treated as true.
    const BRAINFCK_FALSE_VALUE: u8 = 0;

    /// Returns the value recorded in the current memory slot.
    fn get_current_memslot_value(&self) -> u8 {
        self.memory_slots[self.get_current_memslot_index()]
    }

    /// Returns the index (as a usize) of the memslot currently in use.
    fn get_current_memslot_index(&self) -> usize {
        self.current_memslot as usize
    }

    /// Changes the current memslot value to the given value.
    fn set_current_memslot_value(&mut self, new_value: u8) {
        let index = self.get_current_memslot_index();
        self.memory_slots[index] = new_value;
    }

    /// Returns whether or not the value written to the current memory slot
    /// is a value treated as true by the language (any value except 0).
    fn check_current_memslot(&self) -> bool {
        self.get_current_memslot_value() != Self::BRAINFCK_FALSE_VALUE
    }
}
