use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("AHpwncxAnUsYngmKQajpgrRjZP3Gz4ysZiLQqjWZoBWK");

#[program]
pub mod solana_app {
    use super::*;

    pub fn create_player_stats(ctx: Context<CreatePlayerState>) -> Result<()> {
        let player_state = &mut ctx.accounts.player_state;
        
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

#[derive(Accounts)]
pub struct CreatePlayerState<'info> {
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

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;

impl PlayerState {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + 2 // bet
        + 1 // bump
        ;
}  