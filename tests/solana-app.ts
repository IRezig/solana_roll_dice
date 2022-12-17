import { expect } from 'chai';
import { Keypair } from '@solana/web3.js';
import {
  getCurrentRound,
  getPlayerState,
  getWalletKey,
  initAppState,
  initPlayer,
  getLastRound,
  getStats,
  play,
  GAME_PRICE,
  claim,
  goNextRound,
  getBalance
} from './utils';

describe('solana-app', () => {
  const besma = getWalletKey();
  const ju = Keypair.generate();

  it('init accounts', async () => {
    await initAppState();
    await initPlayer(besma);

    const current = await getCurrentRound();
    expect(current.benefits).equal(0);

    const last = await getLastRound();
    expect(last.benefits).equal(0);

    const stats = await getStats();
    expect(stats.totalRolls).equal(0);

    const player = await getPlayerState(besma);
    expect(player.lastClaimedRound).equal(0);
  });

  it('play lose ', async () => {
    await play(besma, 2);

    const current = await getCurrentRound();
    expect(current.benefits).equal(GAME_PRICE);
  });

  it('play win ', async () => {
    await play(besma, 4);

    const player = await getPlayerState(besma);
    expect(player.payback).equal(GAME_PRICE);
  });

  it('claim', async () => {
    await claim(besma);

    const player = await getPlayerState(besma);
    expect(player.payback).equal(0);
  });

  it('go next round', async () => {
    await goNextRound(Date.now());
    await goNextRound(Date.now());
  });

  it('check payment', async () => {
    const balance = await getBalance(besma);
    await play(besma, 2);

    const newBalance = await getBalance(besma);
    expect(newBalance).lt(balance - GAME_PRICE);
  });
});
