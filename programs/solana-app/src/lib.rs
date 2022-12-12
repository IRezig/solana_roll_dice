use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("AHpwncxAnUsYngmKQajpgrRjZP3Gz4ysZiLQqjWZoBWK");

#[program]
pub mod solana_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let player_state = &mut ctx.accounts.player_state;
        let current_round = &mut ctx.accounts.current_round;
        
        current_round.id = 1;
        player_state.bet = 0;
        player_state.bump = *ctx.bumps.get("player_state").unwrap();
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

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = player, 
        space = PlayerState::LEN, 
        seeds = [b"player_state".as_ref(), player.key().as_ref()],
        bump,
    )]
    pub player_state: Account<'info, PlayerState>,
    #[account(
        init,
        payer = player, 
        space = CurrentRound::LEN, 
        seeds = [b"current_round".as_ref(), player.key().as_ref()],
        bump,
    )]
    pub current_round: Account<'info, CurrentRound>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;

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