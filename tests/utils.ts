import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolanaApp } from '../target/types/solana_app';
import { LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import { expect } from 'chai';

export const GAME_PRICE = 0.001 * LAMPORTS_PER_SOL;

/**
 * Seeds
 */
const CURRENT_ROUND_SEED = 'current_round';
const LAST_ROUND_SEED = 'last_round';
const STATS_SEED = 'stats';
const PLAYER_STATE_SEED = 'player_state';

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
  const seeds = [anchor.utils.bytes.utf8.encode(seed)];
  if (pubKey) {
    seeds.push(pubKey.toBuffer());
  }
  const [pda, _] = PublicKey.findProgramAddressSync(seeds, program.programId);
  return pda;
};
export const getCurrentRound = async () =>
  await program.account.currentRound.fetch(addressForSeed(CURRENT_ROUND_SEED));
export const getLastRound = async () =>
  await program.account.lastRound.fetch(addressForSeed(LAST_ROUND_SEED));
export const getStats = async () =>
  await program.account.stats.fetch(addressForSeed(STATS_SEED));
export const getPlayerState = async (pubKey: PublicKey) =>
  await program.account.playerState.fetch(
    addressForSeed(PLAYER_STATE_SEED, pubKey)
  );
export const getWalletKey = () => provider.wallet.publicKey;
export const getBalance = async (pubKey: PublicKey) =>
  provider.connection.getBalance(pubKey);

export const expectValueForKey = async (
  request: Promise<any>,
  key: string,
  value: any
) => {
  expect(await getValueForKey(request, key)).equal(value);
};
export const getValueForKey = async (request: Promise<any>, key: string) => {
  const val = await request;
  return val[key];
};
export const initAppState = async () => {
  const currentRound = addressForSeed(CURRENT_ROUND_SEED);
  const lastRound = addressForSeed(LAST_ROUND_SEED);
  const stats = addressForSeed(STATS_SEED);

  return await program.methods
    .initAppState()
    .accounts({
      player: provider.wallet.publicKey,
      currentRound,
      lastRound,
      stats,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
};

export const initPlayer = async (pubKey: PublicKey) => {
  const playerState = addressForSeed(PLAYER_STATE_SEED, pubKey);
  return await program.methods
    .initPlayer()
    .accounts({
      player: provider.wallet.publicKey,
      playerState,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
};

export const play = async (pubKey: PublicKey, bet: number) => {
  const currentRound = addressForSeed(CURRENT_ROUND_SEED);
  const lastRound = addressForSeed(LAST_ROUND_SEED);
  const stats = addressForSeed(STATS_SEED);
  const playerState = addressForSeed(PLAYER_STATE_SEED, pubKey);

  return await program.methods
    .play(bet)
    .accounts({
      currentRound,
      lastRound,
      stats,
      playerState,
      player: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
};

export const claim = async (pubKey: PublicKey) => {
  const currentRound = addressForSeed(CURRENT_ROUND_SEED);
  const lastRound = addressForSeed(LAST_ROUND_SEED);
  const playerState = addressForSeed(PLAYER_STATE_SEED, pubKey);
  const stats = addressForSeed(STATS_SEED);

  return await program.methods
    .claim()
    .accounts({
      currentRound,
      lastRound,
      playerState,
      stats,
      player: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();
};
