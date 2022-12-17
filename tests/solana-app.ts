import { expect } from 'chai';
import {
  GAME_PRICE,
  claim,
  getBalance,
  getCurrentRound,
  getLastRound,
  getPlayerState,
  getStats,
  getWalletKey,
  initAppState,
  initPlayer,
  play
} from './utils';

describe('solana-app', () => {
  const besma = getWalletKey();
  //const ju = Keypair.generate();

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
    play(besma, 2);
    const current = await getCurrentRound();
    expect(current.id).equal(current.id);
  });

  it('check payment', async () => {
    const b1 = await getBalance(besma);
    await play(besma, 2);

    const b2 = await getBalance(besma);
    expect(b2).lt(b1 - GAME_PRICE);

    await play(besma, 4);

    const b3 = await getBalance(besma);
    expect(b3).gt(b2 - GAME_PRICE);
  });
});
