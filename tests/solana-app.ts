import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolanaApp } from "../target/types/solana_app";
import { PublicKey } from '@solana/web3.js';

describe("solana-app", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaApp as Program<SolanaApp>;
  
  it('can send a new tweet', async () => {
    // const signature = await program.provider.connection.requestAirdrop(player.publicKey, 1000000000);
    // await program.provider.connection.confirmTransaction(signature);
    // const provider = anchor.AnchorProvider.env();
    // anchor.setProvider(provider);
    const pubKey = program.provider.wallet.publicKey;
    console.log('pubKey: ', pubKey)
    
    const [playerState, _] = PublicKey.findProgramAddressSync([
        anchor.utils.bytes.utf8.encode("player-state"),
        pubKey.toBuffer()
      ],
      program.programId
    );
    console.log('playerState', playerState)

    await program.methods
      .createPlayerStats()
      .accounts({
        player: pubKey,
        playerState: playerState,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .transaction()

    const PlayerAccount = await program.account.playerState.fetch(playerState);
  	console.log(PlayerAccount);
  });
});
