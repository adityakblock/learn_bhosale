import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Learn1 } from '../target/types/learn1';

describe('learn1', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Learn1 as Program<Learn1>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
