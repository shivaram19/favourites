use anchor_lang::prelude::*;

declare_id!("BvRrHgb9Vnnfc2BAMJv2cm4FN3952Kfmhg6yZ6eYrj3m");

#[program]
pub mod favourites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
