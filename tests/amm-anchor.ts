import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AmmAnchor } from "../target/types/amm_anchor";


/* 
what do we need :

a account as a signer with authority to initilise the amm config

a user acc to depost token
user -> mint_x
user -> mint_y
user -> ata_x
user -> ata_y

valut to store these token 

vault_x
vault_y

user ata to mint them LP token

user_lp

*/

describe("amm-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ammAnchor as Program<AmmAnchor>;

  const provider = anchor.getProvider();

  const connection = provider.connection;

  const confirm = async ( signature: string ) : Promise<string> => {
    const block = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature,
      ...block
    });

    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );

    return signature;
  };

  



  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
