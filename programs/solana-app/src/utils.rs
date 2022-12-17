use anchor_lang::prelude::*;
use super::*;

pub fn pay<'info>(from: &Signer<'info>, to: &Account<'info, Stats>, amount: u64) -> std::result::Result<(), anchor_lang::prelude::ProgramError> {
	let instruction = anchor_lang::solana_program::system_instruction::transfer(
		&from.key(),
		&to.key(),
		amount,
	);
	anchor_lang::solana_program::program::invoke(
		&instruction,
		&[
			from.to_account_info(),
			to.to_account_info(),
		],
	)
}

pub fn withdraw<'info>(to: &Signer<'info>, from: &Account<'info, Stats>, amount: u64) -> std::result::Result<(), anchor_lang::prelude::ProgramError> {
	let instruction = anchor_lang::solana_program::system_instruction::transfer(
		&from.key(),
		&to.key(),
		amount,
	);
	anchor_lang::solana_program::program::invoke(
		&instruction,
		&[
			from.to_account_info(),
			to.to_account_info(),
		],
	)
}