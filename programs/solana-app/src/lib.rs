use anchor_lang::prelude::*;
mod init;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::entrypoint::ProgramResult;
pub use crate::init::initializer::*;

declare_id!("AHpwncxAnUsYngmKQajpgrRjZP3Gz4ysZiLQqjWZoBWK");

#[program]
pub mod solana_app {
    use super::*;

    pub fn init_current_round(ctx: Context<InitializeAppState>) -> Result<()> {
        init_current_round(ctx);
        Ok(())
    }

    pub fn init_player(ctx: Context<InitializePlayer>) -> Result<()> {
        init_player(ctx);
        Ok(())
    }
}