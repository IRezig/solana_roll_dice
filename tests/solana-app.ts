import { expect } from "chai";
import { getCurrentRound, getPlayerState, getWalletKey, initAppState, initPlayer, getLastRound, getStats  } from "./utils";

describe("solana-app", () => {
  const besma = getWalletKey();

  it("can init app sate", async () => {
    await initAppState();
    expect(initAppState()).to.throw;

    const current = await getCurrentRound();
    expect(current.id).equal(2);
  });

  it("can init player", async () => {
    await initPlayer(besma);
    expect(initPlayer(besma)).to.throw;

    const player = await getPlayerState(besma);
    expect(player.bet).equal(0);
  });
});
