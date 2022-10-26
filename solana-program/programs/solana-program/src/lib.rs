use anchor_lang::prelude::*;

declare_id!("2Mm6nxoHYXr4Fm6YHRCM8pdMJGe5a8vq55pYRGwCxYZj");

#[program]
pub mod solana_program {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("my first program is called! ");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
