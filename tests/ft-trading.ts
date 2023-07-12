import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FtTrading } from "../target/types/ft_trading";

describe("ft-trading", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FtTrading as Program<FtTrading>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
