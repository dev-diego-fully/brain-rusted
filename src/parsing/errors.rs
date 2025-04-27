use super::instruction::Instruction;

/// Returns an error message stating that an attempt was made to close a
/// loop without one being open. The program must receive the status until
/// it encounters the attempted closure.
pub(crate) fn error_mismatched_loop_closing(mismatched: &[Instruction]) -> String {
    format!(
        "Mismatch close loop in: {}",
        Instruction::represents_mismatched(mismatched)
    )
}

/// Returns an error message stating that a loop was not closed properly.
/// Receives the instructions for the loop that was left open.
pub(crate) fn error_unclosed_loop(unclosed: &[Instruction]) -> String {
    format!(
        "Unclosed loop error in: {}",
        Instruction::represents_unclosed(unclosed)
    )
}
