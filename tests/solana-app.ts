import { expect } from "chai";
import { Keypair, PublicKey } from '@solana/web3.js';
import { getCurrentRound, getPlayerState, getWalletKey, initAppState, initPlayer, getLastRound, getStats, play  } from "./utils";

describe("solana-app", () => {

	const besma = getWalletKey();
	const ju = Keypair.generate();

	// it('can init app state', async () => {
	// 	await initAppState()
	// 	expect(initAppState()).to.throw;

	// 	const current = await getCurrentRound();
	// 	expect(current.id).equal(1)

	// 	const last = await getLastRound();
	// 	expect(last.benefits).equal(0)

	// 	const stats = await getStats();
	// 	expect(stats.totalRolls).equal(0)
	// });

	// it('can init player', async () => {
	// 	await initPlayer(besma)
	// 	expect(initPlayer(besma)).to.throw

	// 	const player = await getPlayerState(besma);
	// 	expect(player.lastClaimedRound).equal(0)
	// });

	it('can play', async () => {
		await initAppState()
		await initPlayer(besma)

		await play(besma)
	});

});
