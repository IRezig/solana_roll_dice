use anchor_lang::prelude::*;
use super::*;

pub fn init_player(ctx: Context<InitializePlayer>) {
    let player_state = &mut ctx.accounts.player_state;
    player_state.bump = *ctx.bumps.get("player_state").unwrap();
}

pub fn init_app_state(ctx: Context<InitializeAppState>) {
    let current_round = &mut ctx.accounts.current_round;
    current_round.id = 1;
}

#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        payer = player, 
        space = PlayerState::LEN, 
        seeds = [b"player_state".as_ref(), player.key().as_ref()],
        bump,
    )]
    pub player_state: Account<'info, PlayerState>,

    #[account(mut)]
    pub player: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAppState<'info> {
    #[account(
        init,
        payer = player, 
        space = CurrentRound::LEN, 
        seeds = [b"current_round".as_ref()],
        bump,
    )]
    pub current_round: Account<'info, CurrentRound>,

    #[account(
        init,
        payer = player, 
        space = LastRound::LEN, 
        seeds = [b"last_round".as_ref()],
        bump,
    )]
    pub last_round: Account<'info, LastRound>,

    #[account(
        init,
        payer = player, 
        space = Stats::LEN, 
        seeds = [b"stats".as_ref()],
        bump,
    )]
    pub stats: Account<'info, Stats>,

    #[account(mut)]
    pub player: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

}

const DISCRIMINATOR_LENGTH: usize = 8;
//const PUBLIC_KEY_LENGTH: usize = 32;
//const TIMESTAMP_LENGTH: usize = 8;

impl PlayerState {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 4 // lastClaimedRound
        + 4 // totalClaimed
        + 4 // nbShares
        + 4 // currentRoundShares
        + 4 // lastWonRound
        + 4 // payback
        + 4 // pendingRollId
        + 1 // bump
        ;
} 

impl CurrentRound {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 4 // id
        + 4 // benefits
        + 1 // bump
        ;
}

impl LastRound {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 4 // winners
        + 4 // benefits
        + 4 // totalClaimed
        + 4 // timestamp
        + 1 // bump
        ;
}

impl Stats {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 4 // totalClaimed
        + 4 // totalWinners
        + 4 // totalRolls
        + 1 // bump
        ;
}
