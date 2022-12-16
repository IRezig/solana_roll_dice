import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaApp } from "../target/types/solana_app";
import { PublicKey } from '@solana/web3.js';
import { assert, expect } from "chai";

describe("solana-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaApp as Program<SolanaApp>;
  
  const getPDA = (seed: string, pubKey?: PublicKey) => {
    const seeds = [anchor.utils.bytes.utf8.encode(seed)]
    if (pubKey) {
      seeds.push(pubKey.toBuffer())
    }
    const [pda, _] = PublicKey.findProgramAddressSync(seeds, program.programId)
    return pda
  }

  it('can init app sate', async () => {
    // const signature = await program.provider.connection.requestAirdrop(player.publicKey, 1000000000);
    // await program.provider.connection.confirmTransaction(signature);
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const pubKey = provider.wallet.publicKey;
    const c = getPDA("current_round")

    await program.methods
    .initCurrentRound()
    .accounts({
      player: pubKey,
      currentRound: c,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc()

    let success = false
    try {
      await program.methods
        .initCurrentRound()
        .accounts({
          player: pubKey,
          currentRound: c,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc()
        success = true

      } catch(e){}
    expect(success).to.be.false;

    const current = await program.account.currentRound.fetch(c);
    expect(current.id).equal(2)
  });

  it('can init player', async () => {
    // const signature = await program.provider.connection.requestAirdrop(player.publicKey, 1000000000);
    // await program.provider.connection.confirmTransaction(signature);
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const pubKey = provider.wallet.publicKey;
    const p = getPDA("player_state", pubKey)

    await program.methods
    .initPlayer()
    .accounts({
      player: pubKey,
      playerState: p,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc()

    let success = false
    try {
      await program.methods
        .initPlayer()
        .accounts({
          player: pubKey,
          playerState: p,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc()
        success = true
      } catch(e){}
    
    expect(success).to.be.false;
    const playerAccount = await program.account.playerState.fetch(p);
    expect(playerAccount.bet).equal(0)
  });

});
