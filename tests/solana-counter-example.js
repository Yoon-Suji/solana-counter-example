const anchor = require("@project-serum/anchor");
const { assert } = require("chai");

describe("solana-counter-example", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SolanaCounterExample;
  const counter = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().accounts({
      counterAccount: counter.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([counter]).rpc();

    const counterData = await program.account.counter.fetch(counter.publicKey);
    assert.exists(counterData.count);
    console.log(counterData);
    console.log("Your transaction signature ", tx);
  });

  it("Increments the counter", async () => {
    const NUMBER = "5";
    await program.rpc.increment(new anchor.BN(NUMBER), {
      accounts: {
        counterAccount: counter.publicKey,
      },
    });
    const counterData = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterData.count.toString() === NUMBER);
    console.log(counterData.count);
  });

  it("Resets the counter", async () => {
    const NUMBER = "1";
    await program.rpc.reset(new anchor.BN(NUMBER), {
      accounts: {
        counterAccount: counter.publicKey,
      },
    });
    const counterData = await program.account.counter.fetch(counter.publicKey);
    assert.ok(counterData.count.toString() === NUMBER);
    console.log(counterData.count);
  })
});
