use solana_program::program_error::ProgramError;

pub enum HelloInstruction {
    Increment,
    Decrement,
    Set(),
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<self, ProgramError> {}
}
