import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OpenGrants } from "../target/types/open_grants";

describe("open_grants", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OpenGrants as Program<OpenGrants>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
