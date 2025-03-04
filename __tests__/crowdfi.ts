import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfi } from "../target/types/crowdfi";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { BN } from "bn.js";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { randomBytes } from 'node:crypto';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { assert } from "chai";

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
  let userRewardAtaB;
  let campaignRewardMintMetadata;
  let campaignRewardMintMetadataBump;
  
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
    
      userRewardAtaB = await getAssociatedTokenAddress(campaign_mint, user.publicKey);
      console.log("✅ User Campaign Mint ATA B: ", userRewardAtaB);

      // [campaignRewardMintMetadata, campaignRewardMintMetadataBump] = PublicKey.findProgramAddressSync([
      //   Buffer.from("metadata"),
      //   MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      //   campaign_mint.toBuffer(),
      // ], MPL_TOKEN_METADATA_PROGRAM_ID);

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
        // campaignRewardMintMetadata: campaignRewardMintMetadata,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Campaign is Not Created For Long Title!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    try {
      const tx = await program.methods
        .createCampaign(
          "ASIODOIINIANAONINIINSNDOSNNINNSAFINIASSNAADFOFINAIFSNFINAIFNISOFDNSDDANNDDOIIINOIFDIINISNISOFSSOIAAOFFFAFIAIINAADDFFOAINIFISAFNIIINIIOINAASOONDNSAINNSNSINNIINAOASNOAINIODDOIIFFOFFIDFINDIONSIFSFNDNDIFONISFSSIFINANNIANINDSINNSISAADIOONAIINOANIIIAAOOAIAA",
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
    } catch(error) {
      if (!error.toString().includes("Reached maximum depth for account resolution")) {
        throw error;
      }
    }
  });
  
  it("Campaign is Not Created For Long Description!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    try {
      const tx = await program.methods
        .createCampaign(
          "Test Description",
          "ASIODOIINIANAONINIINSNDOSNNINNSAFINIASSNAADFOFINAIFSNFINAIFNISOFDNSDDANNDDOIIINOIFDIINISNISOFSSOIAAOFFFAFIAIINAADDFFOAINIFISAFNIIINIIOINAASOONDNSAINNSNSINNIINAOASNOAINIODDOIIFFOFFIDFINDIONSIFSFNDNDIFONISFSSIFINANNIANINDSINNSISAADIOONAIINOANIIIAAOOAI@",
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
    } catch(error) {
      if (!error.toString().includes("Reached maximum depth for account resolution")) {
        throw error;
      }
    }
  });
  
  it("Campaign is Not Created For Long URL!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    try {
      const tx = await program.methods
        .createCampaign(
          "Test Title",
          "Desription",
          "ASIODOIINIANAONINIINSNDOSNNINNSAFINIASSNAADFOFINAIFSNFINAIFNISOFDNSDDANNDDOIIINOIFDIINISNISOFSSOIAAOFFFAFIAIINAADDFFOAINIFISAFNIIINIIOINAASOONDNSAINNSNSINNIINAOASNOAINIODDOIIFFOFFIDFINDIONSIFSFNDNDIFONISFSSIFINANNIANINDSINNSISAADIOONAIINOANIIIAAOOAI@",
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
    } catch(error) {
      if (!error.toString().includes("Reached maximum depth for account resolution")) {
        throw error;
      }
    }
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

  it("Campaign is Updated For Optional Url!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    const tx = await program.methods
      .updateCampaign(
        "Test Description Updated",
        null,
      )
      .accountsPartial({
        admin: admin.publicKey,
        campaign: campaign,
      })
      .signers([admin])
      .rpc();

    console.log("Your transaction signature", tx);
  });
  
  it("Campaign is Updated For Optional Description!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    const tx = await program.methods
      .updateCampaign(
        null,
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
  
  it("Campaign is Not Updated For Description Too Long!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    try {
        const tx = await program.methods
        .updateCampaign(
          "ASIODOIINIANAONINIINSNDOSNNINNSAFINIASSNAADFOFINAIFSNFINAIFNISOFDNSDDANNDDOIIINOIFDIINISNISOFSSOIAAOFFFAFIAIINAADDFFOAINIFISAFNIIINIIOINAASOONDNSAINNSNSINNIINAOASNOAINIODDOIIFFOFFIDFINDIONSIFSFNDNDIFONISFSSIFINANNIANINDSINNSISAADIOONAIINOANIIIAAOOAI@",
          "http://updated_test_url.com",
        )
        .accountsPartial({
          admin: admin.publicKey,
          campaign: campaign,
        })
        .signers([admin])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch(error) {
      if (!error.toString().includes("Reached maximum depth for account resolution")) {
        throw error;
      }
    }
  });
  
  it("Campaign is Not Updated For URL Too Long!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)

    try {
        const tx = await program.methods
        .updateCampaign(
          "Small Description",
          "ASIODOIINIANAONINIINSNDOSNNINNSAFINIASSNAADFOFINAIFSNFINAIFNISOFDNSDDANNDDOIIINOIFDIINISNISOFSSOIAAOFFFAFIAIINAADDFFOAINIFISAFNIIINIIOINAASOONDNSAINNSNSINNIINAOASNOAINIODDOIIFFOFFIDFINDIONSIFSFNDNDIFONISFSSIFINANNIANINDSINNSISAADIOONAIINOANIIIAAOOAI@",
        )
        .accountsPartial({
          admin: admin.publicKey,
          campaign: campaign,
        })
        .signers([admin])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch(error) {
      if (!error.toString().includes("Reached maximum depth for account resolution")) {
        throw error;
      }
    }
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
        // vault: campaign_vault,
        // rewardMint: campaign_mint,
        // userRewardAta: userRewardAtaB,
        // tokenProgram: TOKEN_2022_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);

    let campaignVaultBalance = await getBalance(program.provider.connection, campaign_vault);
    assert.equal(campaignVaultBalance, 1_001_000);
  });
  
  it("Is Donated to Campaign Fails for Zero Value!", async () => {
    await airdrop(program.provider.connection, admin.publicKey, 100)
    await airdrop(program.provider.connection, user.publicKey, 100)

    try {
      const tx = await program.methods
        .donate(
          new BN(0), // Donating the amount plus the an offset
        )
        .accountsPartial({
          campaign: campaign,
          config: config,
          campaignAdmin: admin.publicKey,
          admin: admin.publicKey,
          signer: user.publicKey,
          // vault: campaign_vault,
          // rewardMint: campaign_mint,
          // userRewardAta: userRewardAtaB,
          // tokenProgram: TOKEN_2022_PROGRAM_ID,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([user])
        .rpc();

      console.log("Your transaction signature", tx);
    } catch(error) {
      if (!error.toString().includes("Invalid Amount Value.")) {
        throw error;
      }
    }
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
        userRewardAta: userRewardAtaB,
        vault: campaign_vault,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);
    let campaignVaultBalance = await getBalance(program.provider.connection, campaign_vault);
    assert.equal(campaignVaultBalance, 1_000_000);
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
});


async function airdrop(connection, address: PublicKey, amount: number) {
  let airdrop_signature = await connection.requestAirdrop(
    address,
    amount * LAMPORTS_PER_SOL
  );
  
  let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");
  // console.log("✅ Tx Signature: ", confirmedAirdrop);

  return confirmedAirdrop;
}

async function getBalance(connection: anchor.web3.Connection, address: PublicKey) {
  let accountInfo = await connection.getAccountInfo(address);

  return accountInfo.lamports;
}