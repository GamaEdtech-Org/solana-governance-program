import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GamaedtechProgram } from "../target/types/gamaedtech_program";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import * as assert from "assert";

describe("gamaedtech_program", () => {
  it("just logs ok", async () => {
    console.log("ok");
  });
  // // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GamaedtechProgram as Program<GamaedtechProgram>;

  const creator = Keypair.generate();

  //For proposal creation we need more than 50_000_000 tokens staked
  it('Stake 50_000_000 tokens', async () => {
    // Airdrop SOL to the creator
    const airdropSignature = await provider.connection.requestAirdrop(
      creator.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSignature);
  })


  it("Creates a proposal", async () => {
     // Generate a new keypair for the proposal account
    const proposalKeypair = Keypair.generate();

 

    // Proposal details
    const title = "Test Proposal";
    const brief = "This is a test proposal.";
    const cate = "General";
    const reference = "Reference123";
    const amount = new anchor.BN(1000);

  //   // Derive PDA for the proposal
    const [proposalPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("proposal"), creator.publicKey.toBuffer()],
      program.programId
    );

  //   // Call the createProposal instruction
    await program.methods
      .createProposal(title, brief, cate, reference, amount)
      .accounts({
        proposal: proposalPda,
        user: creator.publicKey,
      })
      .signers([creator])
      .rpc();

  //   // Fetch the created proposal account
    const proposal = await program.account.proposal.fetch(proposalPda);

  //   // Log proposal info
    console.log("Proposal Owner:", proposal.owner.toString());
    console.log("Title:", proposal.title);
    console.log("Brief:", proposal.brief);
    console.log("Category:", proposal.cate);
    console.log("Reference:", proposal.reference);
    console.log("Amount:", proposal.amount.toString());
    console.log("Agree Votes:", proposal.agreeVotes.toString());
    console.log("Disagree Votes:", proposal.disagreeVotes.toString());
    console.log("Created At:", proposal.createdAt.toString());
    console.log("Expires At:", proposal.expiresAt.toString());

  //   // Assertions
    assert.strictEqual(proposal.owner.toString(), creator.publicKey.toString());
    assert.strictEqual(proposal.title, title);
    assert.strictEqual(proposal.brief, brief);
    assert.strictEqual(proposal.cate, cate);
    assert.strictEqual(proposal.reference, reference);
    assert.strictEqual(proposal.amount.toString(), amount.toString());
    assert.strictEqual(proposal.agreeVotes.toString(), "0");
    assert.strictEqual(proposal.disagreeVotes.toString(), "0");
  });
});
