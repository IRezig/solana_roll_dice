mod init;
mod logic;

pub use crate::init::*;
pub use crate::logic::*;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;
use anchor_lang::solana_program::system_program;

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
    
    pub fn play(ctx: Context<Play>, bet: u8) -> Result<()> {
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        let current_round = &mut ctx.accounts.current_round;
        let last_round = &mut ctx.accounts.last_round;
        let player_state = &mut ctx.accounts.player_state;
        let stats = &mut ctx.accounts.stats;
        logic::play(bet, current_round, last_round, player_state, stats, current_timestamp);
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        let current_round = &mut ctx.accounts.current_round;
        let last_round = &mut ctx.accounts.last_round;
        let player_state = &mut ctx.accounts.player_state;
        let stats = &mut ctx.accounts.stats;
        logic::claim(current_round, last_round, player_state, stats);
        Ok(())
    }

    pub fn go_next_round(ctx: Context<GoNextRound>, now: i64) -> Result<()> {
        let current_round = &mut ctx.accounts.current_round;
        let last_round = &mut ctx.accounts.last_round;
        let stats = &mut ctx.accounts.stats;

        logic::go_next_round(current_round, last_round, stats, now);
        Ok(())
    }
}

#[account]
pub struct PlayerState {
    pub last_claimed_round: u32,
    pub total_claimed: u32,
    pub nb_shares: u32,
    pub current_round_shares: u32,
    pub last_won_round: u32,
    pub payback: u32,
    pub pending_roll_id: u32,
    pub bump: u8,
}

#[account]
pub struct CurrentRound {
    pub id: u32,
    pub benefits: u32,
    pub bump: u8,
}

#[account]
pub struct LastRound {
    pub winners: u32,
    pub benefits: u32,
    pub total_claimed: u32,
    pub timestamp: i64,
    pub bump: u8,
}

#[account]
pub struct Stats {
    pub total_claimed: u32,
    pub total_winners: u32,
    pub total_rolls: u32,
    pub bump: u8,
}