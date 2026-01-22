import * as anchor from "@coral-xyz/anchor";
import {
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountIdempotentInstruction,
  TOKEN_2022_PROGRAM_ID
} from "@solana/spl-token";

(async () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.GamaedtechProgram;
  const mint = new anchor.web3.PublicKey("GeutGuhcTYRf4rkbZmWDMEgjt5jHyJN4nHko38GJjQhv");

  // ---- PDA reward authority ----
  const [rewardAuthority, rewardBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("reward-authority")],
    program.programId
  );

  // ---- Create ATA for reward ----
  const rewardAta = getAssociatedTokenAddressSync(
    mint,
    rewardAuthority,
    true,                  // allow owner off-curve
    TOKEN_2022_PROGRAM_ID  // Token-2022
  );

  // Instruction to create ATA (idempotent = safe if already exists)
  const ix = createAssociatedTokenAccountIdempotentInstruction(
    program.provider.publicKey, // payer
    rewardAta,                   // ATA
    rewardAuthority,             // owner of ATA
    mint,                       // token
    TOKEN_2022_PROGRAM_ID       // Token-2022 program
  );

  // Send transaction
  const tx = new anchor.web3.Transaction().add(ix);

  const sig = await program.provider.sendAndConfirm(tx);
  
  console.log("Reward ATA created:", rewardAta.toString());
  console.log("Tx:", sig);
})();
