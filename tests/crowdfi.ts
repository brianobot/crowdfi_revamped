import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfi } from "../target/types/crowdfi";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { BN } from "bn.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("crowdfi", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Crowdfi as Program<Crowdfi>;

  let config;
  let campaign;
  let config_bump;
  let campaign_vault
  let campaign_bump;
  let campaign_mint;
  let mint_bump;
  
  const updateAuthority = anchor.web3.Keypair.generate();

  before(async () => {
      [config, config_bump] = PublicKey.findProgramAddressSync([
        Buffer.from("config"),
      ], program.programId);
      console.log("✅ Config Account Address: ", config);
      
      [campaign, campaign_bump] = PublicKey.findProgramAddressSync([
        Buffer.from("campaign"),
        Buffer.from("Test title"),
        updateAuthority.publicKey.toBuffer(),
      ], program.programId);
      console.log("✅ Campaign Account Address: ", campaign);

      [campaign_vault, config_bump] = PublicKey.findProgramAddressSync([
        Buffer.from("campaign_vault"),
        campaign.toBuffer(),
      ], program.programId);
      console.log("✅ Campaign Vault Account Address: ", campaign_vault);
      
      [campaign_mint, mint_bump] = PublicKey.findProgramAddressSync([
        Buffer.from("reward_mint"),
        campaign.toBuffer(),
      ], program.programId);
      console.log("✅ Campaign Mint Account Address: ", campaign_mint);
  });

  it("Config Is Initialized!", async () => {
    await airdrop(program.provider.connection, updateAuthority.publicKey, 100)

    const tx = await program.methods
      .initializeConfig(
        new BN(1000),
        new BN(1000),
      )
      .accountsPartial({
        admin: updateAuthority.publicKey,
        config: config,
      })
      .signers([updateAuthority])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Campaign is Created!", async () => {
    await airdrop(program.provider.connection, updateAuthority.publicKey, 100)

    const tx = await program.methods
      .createCampaign(
        "Test title",
        "Test Description",
        "http://test_url.com",
        new BN(10),
        new BN(1_000_000),
        new BN(1_000_000),
      )
      .accountsPartial({
        user: updateAuthority.publicKey,
        campaign: campaign,
        rewardMint: campaign_mint,
        campaignVault: campaign_vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([updateAuthority])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Campaign is Updated!", async () => {
    await airdrop(program.provider.connection, updateAuthority.publicKey, 100)

    const tx = await program.methods
      .updateCampaign(
        "Test Description Updated",
        "http://updated_test_url.com",
      )
      .accountsPartial({
        user: updateAuthority.publicKey,
        campaign: campaign,
      })
      .signers([updateAuthority])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Is Donated to Campaign!", async () => {
    await airdrop(program.provider.connection, updateAuthority.publicKey, 100)

    const tx = await program.methods
      .donate(
        new BN(1_000_000),
      )
      .accountsPartial({
        user: updateAuthority.publicKey,
        admin: updateAuthority.publicKey,
        campaign: campaign,
        campaignVault: campaign_vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([updateAuthority])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Is Refunded from Campaign!", async () => {
    await airdrop(program.provider.connection, updateAuthority.publicKey, 100)

    const tx = await program.methods
      .refund(
        new BN(1_000_000),
      )
      .accountsPartial({
        user: updateAuthority.publicKey,
        campaign: campaign,
        campaignVault: campaign_vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([updateAuthority])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});


async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  console.log("✍🏾 Airdrop Signature: ", airdrop_signature);

  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");

  console.log(`🪂 Airdropped ${amount} SOL to ${address.toBase58()}`);
  console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}