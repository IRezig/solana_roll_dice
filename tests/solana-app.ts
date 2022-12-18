import { expect } from 'chai';
import {
  GAME_PRICE,
  ROUND_DURATION,
  claim,
  expectValueForKey,
  getBalance,
  getCurrentRound,
  getLastRound,
  getPlayerState,
  getStats,
  getValueForKey,
  getWalletKey,
  initAppState,
  initPlayer,
  play,
  sleep
} from './utils';

describe('solana-app', () => {
  const besma = getWalletKey();
  //const ju = Keypair.generate();

  it('init accounts', async () => {
    await initAppState();
    await initPlayer(besma);

    await expectValueForKey(getCurrentRound(), 'benefits', 0);
    await expectValueForKey(getLastRound(), 'benefits', 0);
    await expectValueForKey(getStats(), 'totalRolls', 0);
    await expectValueForKey(getPlayerState(besma), 'lastClaimedRound', 0);
  });

  it('play lose ', async () => {
    const prev = await getValueForKey(getCurrentRound(), 'benefits');
    await play(besma, 2);
    await expectValueForKey(getCurrentRound(), 'benefits', prev + GAME_PRICE);
  });

  it('play win ', async () => {
    const prev = await getValueForKey(getPlayerState(besma), 'payback');
    await play(besma, 4);
    await expectValueForKey(
      getPlayerState(besma),
      'payback',
      prev + GAME_PRICE
    );
  });

  it('claim win payback', async () => {
    await claim(besma);
    await expectValueForKey(getPlayerState(besma), 'payback', 0);
  });

  it('go next round', async () => {
    const prev = await getValueForKey(getCurrentRound(), 'id');
    play(besma, 4);
    await sleep(ROUND_DURATION);
    await expectValueForKey(getCurrentRound(), 'id', prev + 1);
  });

  it('claim', async () => {
    const prev = await getValueForKey(
      getPlayerState(besma),
      'lastClaimedRound'
    );
    await claim(besma);
    await expectValueForKey(
      getPlayerState(besma),
      'lastClaimedRound',
      prev + 1
    );
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
