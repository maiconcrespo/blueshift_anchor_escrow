use anchor_lang::prelude::*;

declare_id!("82rMaMx8zoTocAJr46KGUCXTzTxanGZdpXrHGJ3D54Hx");

#[program]
pub mod blueshift_anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
