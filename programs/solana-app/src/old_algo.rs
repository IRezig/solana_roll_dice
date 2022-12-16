const GAME_PRICE: u32 = 1000000;
//const ROUND_DURATION: u32 = 24 * 7 * 3600 * 1000;

struct PlayerState {
    pub last_claimed_round: u32,
    pub total_claimed: u32,
    pub total_shares: u32,
    pub current_round_shares: u32,
    pub last_won_round: u32,
}

struct CurrentRound {
    pub id: u32,
    pub winners: u32,
    pub benefits: u32,
}

struct LastRound {
    pub winners: u32,
    pub benefits: u32,
    pub total_claimed: u32,
    pub timestamp: u32,
}

fn play(
    current_round: &mut CurrentRound,
    last_round: &mut LastRound,
    player_state: &mut PlayerState,
) {
    claim(current_round, last_round, player_state);
    go_next_round(current_round, last_round);
    reset_current_round_shares(current_round, player_state);
    let win = get_random_number();
    if win == 2 {
        // WIN
		player_state.payback += GAME_PRICE;
        player_state.total_shares += 1;
        player_state.current_round_shares += 1;
        player_state.last_won_round = current_round.id;
        current_round.winners += 1;
    } else {
        // LOSE
        current_round.benefits += GAME_PRICE;
    }
	// TODO: NOTIFY FRONT WITH AN EVENT
}

fn claim(
    current_round: &mut CurrentRound,
    last_round: &mut LastRound,
    mut player_state: &mut PlayerState,
) {
    let claimable = get_claimable_amount(current_round, last_round, player_state);
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

fn go_next_round(mut current_round: &mut CurrentRound, mut last_round: &mut LastRound) {
	// TODO: RETRIEVE CURRENT DATE
    let now = 1000000;
    if now > 0 && current_round.winners > 0 {
        let last_unclaimed_benefits: u32 = last_round.benefits - last_round.total_claimed;
        last_round.benefits = last_unclaimed_benefits + current_round.benefits;
        last_round.winners = current_round.winners;
        last_round.total_claimed = 0;
        current_round.benefits = 0;
    }
    last_round.timestamp = now;
    current_round.id += 1;
}

fn reset_current_round_shares(current_round: &CurrentRound, player_state: &mut PlayerState) {
    if current_round.id > player_state.last_won_round {
        player_state.current_round_shares = 0;
    }
}

fn get_claimable_amount(
    current_round: &CurrentRound,
    last_round: &LastRound,
    player_state: &PlayerState,
) -> u32 {
	if last_round.winners == 0 || player_state.last_claimed_round == current_round.id {
		return player_state.payback;
	}
    let mut shares: u32 = player_state.total_shares;
    if player_state.current_round_shares > 0 && player_state.last_won_round == current_round.id {
        shares -= player_state.current_round_shares;
    }
    return last_round.benefits / last_round.winners * shares;
}

fn get_random_number() -> u32 {
    return 4; // TODO RETRIEVE RANDOM NUMBER
}
