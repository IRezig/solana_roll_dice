import { expect } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
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
} from "./utils";

describe("solana-app", () => {
  const besma = getWalletKey();
  const ju = Keypair.generate();

  it("can play", async () => {
    await initAppState();
    await initPlayer(besma);

    await play(besma);

    const current = await getCurrentRound();
    expect(current.benefits).equal(GAME_PRICE);

    const last = await getLastRound();
    expect(last.benefits).equal(0);

    const stats = await getStats();
    expect(stats.totalRolls).equal(0);

    const player = await getPlayerState(besma);
    expect(player.lastClaimedRound).equal(0);
  });
});
