import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaApp } from "../target/types/solana_app";
import { PublicKey } from '@solana/web3.js';

/**
 * Seeds
 */
const CURRENT_ROUND_SEED = "current_round"
const PLAYER_STATE_SEED = "player_state"

/**
 * Basic setup
 */
const program = anchor.workspace.SolanaApp as Program<SolanaApp>;
const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

/**
 * Helpers
 */
export const addressForSeed = (seed: string, pubKey?: PublicKey) => {
	const seeds = [anchor.utils.bytes.utf8.encode(seed)]
	if (pubKey) {
		seeds.push(pubKey.toBuffer())
	}
	const [pda, _] = PublicKey.findProgramAddressSync(seeds, program.programId)
	return pda
}
export const getCurrentRound = async () => await program.account.currentRound.fetch(addressForSeed(CURRENT_ROUND_SEED))
export const getPlayerState = async (pubKey: PublicKey) => await program.account.playerState.fetch(addressForSeed(PLAYER_STATE_SEED, pubKey))
export const getWalletKey = () => provider.wallet.publicKey

export const initAppState = async () => {
    const currentRound = addressForSeed(CURRENT_ROUND_SEED)
	return await program.methods.initAppState().accounts({
		player: provider.wallet.publicKey,
		currentRound,
		systemProgram: anchor.web3.SystemProgram.programId,
	}).rpc()
}
export const initPlayer = async (pubKey: PublicKey) => {
    const playerState = addressForSeed(PLAYER_STATE_SEED, pubKey)
	return await program.methods.initPlayer().accounts({
		player: provider.wallet.publicKey,
		playerState,
		systemProgram: anchor.web3.SystemProgram.programId,
	}).rpc()
}

export const airdrop = async (pubKey: PublicKey, amount = 1000000000) => {
	const signature = await program.provider.connection.requestAirdrop(pubKey, amount);
	await program.provider.connection.confirmTransaction(signature);
}