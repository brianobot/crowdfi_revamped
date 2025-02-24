import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfi } from "../target/types/crowdfi";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { BN } from "bn.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { randomBytes } from 'node:crypto';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, getMint, getAssociatedTokenAddress } from "@solana/spl-token";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

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
  let userRewardAta;
  
  const admin = anchor.web3.Keypair.generate();
  const user = anchor.web3.Keypair.generate();

  const seed = new BN(randomBytes(8));

  before(async () => {
      [config, config_bump] = PublicKey.findProgramAddressSync([
        Buffer.from("config"),
        seed.toArrayLike(Buffer, "le", 8),
      ], program.programId);
      console.log("✅ Config Account Address: ", config);
      
      [campaign, campaign_bump] = PublicKey.findProgramAddressSync([
        Buffer.from("campaign"),
        Buffer.from("Test title"),
        admin.publicKey.toBuffer(),
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

      [userRewardAta, mint_bump] = PublicKey.findProgramAddressSync([
        user.publicKey.toBuffer(),
        campaign_mint.toBuffer(),
        TOKEN_2022_PROGRAM_ID.toBuffer()

      ], ASSOCIATED_TOKEN_PROGRAM_ID);
      console.log("✅ User Campaign Mint Associated Token Account Address: ", userRewardAta);
  });

  it("Config Is Initialized!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    const tx = await program.methods
      .initializeConfig(
        seed, // Seed
        new BN(1000), // max_duration
        new BN(1000), // max_amount
      )
      .accountsPartial({
        admin: admin.publicKey,
        config: config,
      })
      .signers([admin])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Campaign is Created!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

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
        admin: admin.publicKey,
        config: config,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Campaign is Updated!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    const tx = await program.methods
      .updateCampaign(
        "Test Description Updated",
        "http://updated_test_url.com",
      )
      .accountsPartial({
        admin: admin.publicKey,
        campaign: campaign,
      })
      .signers([admin])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Is Donated to Campaign!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)
    await airdrop(program.provider.connection, user.publicKey, 100)

    const tx = await program.methods
      .donate(
        new BN(1_001_000), // Donating the amount plus the an offset
      )
      .accountsPartial({
        campaign: campaign,
        config: config,
        campaignAdmin: admin.publicKey,
        admin: admin.publicKey,
        signer: user.publicKey,
        campaignVault: campaign_vault,
        campaignRewardMint: campaign_mint,
        // userRewardAta: userRewardAta,
        systemProgram: SYSTEM_PROGRAM_ID,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Is Refunded from Campaign!", async () => {
    await airdrop(program.provider.connection, user.publicKey, 100)

    const tx = await program.methods
      .refund(
        new BN(1_000),
      )
      .accountsPartial({
        signer: user.publicKey,
        campaign: campaign,
        campaignVault: campaign_vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Campaign is Closed!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    const tx = await program.methods
      .closeCampaign()
      .accountsPartial({
        signer: admin.publicKey,
        config: config,
        campaign: campaign,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  // it("Pause to inspect", (done) => {
  //   setTimeout(() => done(), 999_000);
  // }).timeout(999_000);
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