use anchor_lang::prelude::*;

pub mod definitions;
pub mod state;

pub use definitions::*;
pub use state::*;

declare_id!("EdiAD6MqML7e4DnfR85Pdbj15m7zyQqGaM8koJhGQq1j");

#[program]
pub mod samizdat {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
