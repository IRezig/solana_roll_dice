
use anchor_lang::prelude::*;
use super::*;

const GAME_PRICE: u32 = 1000000;
//const ROUND_DURATION: u32 = 24 * 7 * 3600 * 1000;

#[derive(Accounts)]
pub struct Play<'info> {
    #[account(
        mut,
        space = CurrentRound::LEN, 
        seeds = [b"current_round".as_ref()],
        bump,
    )]
    pub current_round: Account<'info, CurrentRound>,

    #[account(
        mut,
        space = LastRound::LEN, 
        seeds = [b"last_round".as_ref()],
        bump,
    )]
    pub last_round: Account<'info, LastRound>,

    #[account(
        mut,
        space = Stats::LEN, 
        seeds = [b"stats".as_ref()],
        bump,
    )]
    pub stats: Account<'info, Stats>,

    #[account(
        mut,
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

pub fn play(ctx: Context<Play>, bet: u8) {
	let current_round = &mut ctx.accounts.current_round;
	let last_round = &mut ctx.accounts.last_round;
	let player_state = &mut ctx.accounts.player_state;
	let stats = &mut ctx.accounts.stats;

	_play(bet, current_round, last_round, player_state, stats);
}

fn _play(
	bet: u8,
    current_round: &mut CurrentRound,
    last_round: &mut LastRound,
    player_state: &mut PlayerState,
    stats: &mut Stats,
) {
    claim(current_round, last_round, player_state, stats);
    go_next_round(current_round, last_round, stats);
    reset_current_round_shares(current_round, player_state);
    let win = get_random_number();
    if win == 2 {
        // WIN
		player_state.payback += GAME_PRICE;
        player_state.nb_shares += 1;
        player_state.current_round_shares += 1;
        player_state.last_won_round = current_round.id;
        stats.total_winners += 1;
    } else {
        // LOSE
        current_round.benefits += GAME_PRICE;
    }
	// TODO: NOTIFY FRONT WITH AN EVENT
}

fn claim(
    current_round: &mut CurrentRound,
    last_round: &mut LastRound,
    player_state: &mut PlayerState,
    stats: &mut Stats,
) {
    let claimable = get_claimable_amount(current_round, last_round, player_state, stats);
    if claimable <= 0 {
        return;
    }
	// TODO: TRANSFER FUNDS
	if claimable > player_state.payback {
		last_round.total_claimed += claimable;
		player_state.total_claimed += claimable;
		player_state.last_claimed_round = current_round.id;
	}
	player_state.payback = 0;
}

fn go_next_round(
    current_round: &mut CurrentRound, 
    last_round: &mut LastRound, 
    stats: &mut Stats
) {
	// TODO: RETRIEVE CURRENT DATE
    let now = 1000000;
    if now > 0 && stats.total_winners > 0 {
        let last_unclaimed_benefits: u32 = last_round.benefits - last_round.total_claimed;
        last_round.benefits = last_unclaimed_benefits + current_round.benefits;
        last_round.winners = stats.total_winners;
        last_round.total_claimed = 0;
        current_round.benefits = 0;
    }
    last_round.timestamp = now;
    current_round.id += 1;
}

fn reset_current_round_shares(
	current_round: &CurrentRound,
	player_state: &mut PlayerState
) {
    if current_round.id > player_state.last_won_round {
        player_state.current_round_shares = 0;
    }
}

fn get_claimable_amount(
    current_round: &CurrentRound,
    last_round: &LastRound,
    player_state: &PlayerState,
    stats: &Stats,
) -> u32 {
	if last_round.winners == 0 || player_state.last_claimed_round == current_round.id {
		return player_state.payback;
	}
    let mut shares: u32 = stats.total_winners;
    if player_state.current_round_shares > 0 && player_state.last_won_round == current_round.id {
        shares -= player_state.current_round_shares;
    }
    return last_round.benefits / last_round.winners * shares;
}

fn get_random_number() -> u32 {
    return 4; // TODO RETRIEVE RANDOM NUMBER
}