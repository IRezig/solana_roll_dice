use anchor_lang::prelude::*;
mod init;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::entrypoint::ProgramResult;
pub use crate::init::*;

declare_id!("AHpwncxAnUsYngmKQajpgrRjZP3Gz4ysZiLQqjWZoBWK");

#[program]
pub mod solana_app {
    use super::*;

    pub fn init_app_state(ctx: Context<InitializeAppState>) -> Result<()> {
        init::init_app_state(ctx);
        Ok(())
    }

    pub fn init_player(ctx: Context<InitializePlayer>) -> Result<()> {
        init::init_player(ctx);
        Ok(())
    }
}


#[account]
pub struct PlayerState {
    pub bet: u16,
    pub bump: u8,
}

#[account]
pub struct CurrentRound {
    pub id: u16,
    pub bump: u8,
}
