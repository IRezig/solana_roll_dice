use anchor_lang::prelude::*;
use super::*;

pub fn init_player(ctx: Context<InitializePlayer>) {
    let player_state = &mut ctx.accounts.player_state;
    player_state.bet = 0;
    player_state.bump = *ctx.bumps.get("player_state").unwrap();
}

pub fn init_app_state(ctx: Context<InitializeAppState>) {
    let current_round = &mut ctx.accounts.current_round;
    current_round.id = 2;
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

    #[account(mut)]
    pub player: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

}

const DISCRIMINATOR_LENGTH: usize = 8;
//const PUBLIC_KEY_LENGTH: usize = 32;
//const TIMESTAMP_LENGTH: usize = 8;

impl PlayerState {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + 2 // bet
        + 1 // bump
        ;
} 

impl CurrentRound {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + 2 // id
        + 1 // bump
        ;
}
