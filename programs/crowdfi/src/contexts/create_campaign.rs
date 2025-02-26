use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::{
            self, instructions::{
                CreateMetadataAccountV3Cpi, 
                CreateMetadataAccountV3CpiAccounts, 
                CreateMetadataAccountV3InstructionArgs
            }, 
            types::DataV2
        }, MetadataAccount}, 
        token_interface::{
            Mint, 
            TokenInterface
        }
    };
use anchor_spl::metadata::Metadata;

use crate::state::{Campaign, Config};
use crate::error::CrowdfiError;


#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer = admin,
        space = 8 + Campaign::INIT_SPACE,
        seeds = [b"campaign", title.as_bytes(), admin.key().as_ref()],
        bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        mut,
        seeds = [b"campaign_vault", campaign.key().as_ref()],
        bump,
    )]
    // this would still be owned by my program
    pub vault: SystemAccount<'info>,
    #[account(
        init,
        payer = admin,
        mint::decimals = 6,
        // all account types in the anchor account type implement the AccountSerialize and AccountDeserialize
        // trait, among this trait methods is the key() method which returns the Pubkey (address) of the account
        // in question
        mint::authority = campaign.key(),
        seeds = [b"reward_mint", campaign.key().as_ref()],
        bump,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    // #[account(
    //     seeds = [
    //         b"metadata",
    //         metadata_program.key().as_ref(),
    //         campaign_reward_mint.key().as_ref(),
    //     ],
    //     seeds::program = metadata_program.key(),
    //     bump,
    // )]
    // pub reward_mint_metadata: Account<'info, MetadataAccount>,
    pub system_program: Program<'info, System>,
    // pub metadata_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> CreateCampaign<'info> {
    pub fn init(&mut self, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64, bumps: &CreateCampaignBumps) -> Result<()> {
        require!(title.len() <= 250, CrowdfiError::CampaignTitleIsTooLong);
        require!(description.len() <= 250, CrowdfiError::CampaignDescriptionIsTooLong);
        require!(url.len() <= 250, CrowdfiError::CampaignURLIsTooLong);

        self.campaign.set_inner( Campaign {
            admin: self.admin.key(),
            title,
            description,
            url,
            target_amount,
            current_amount: 0,
            start_timestamp,
            end_timestamp,
            bump: bumps.campaign,
            vault_bump: bumps.vault, 
            reward_mint_bump: bumps.reward_mint,
            is_completed: false
        });
        Ok(())
    }

    pub fn init_mint_metadata(&mut self, title: String) -> Result<()> {
        // let name = title;
        // let symbol = String::from("NNN");
        // let uri = String::from("https://brianobot.github.io/");

        // let seeds = [
        //     b"metadata",
        //     self.reward_mint.to_account_info().key.as_ref(),
        //     mpl_token_metadata::ID.as_ref(),
        // ];

        // let signer_seeds = &[&seeds[..]];

        // let cpi_program = self.token_program.to_account_info();

        // // metadata,
        // // mint,
        // // mint_authority,
        // // update_authority: (update_authority, true),
        // // payer,
        // // system_program,
        // // rent: None,

        // let cpi_accounts = CreateMetadataAccountV3CpiAccounts {
        //     metadata: &self.reward_mint_metadata.to_account_info(),
        //     mint: &self.reward_mint.to_account_info(),
        //     mint_authority: &self.campaign.to_account_info(),
        //     payer: &self.admin.to_account_info(),
        //     update_authority: (&self.campaign.to_account_info(), false),
        //     system_program: &self.system_program.to_account_info(),
        //     rent: None,
        // };

        // let args = CreateMetadataAccountV3InstructionArgs {
        //     data: DataV2 {
        //         name,
        //         symbol,
        //         uri,
        //         seller_fee_basis_points: 0,
        //         creators: None,
        //         collection: None,
        //         uses: None,
        //     },
        //     is_mutable: true,
        //     collection_details: None,
        // };

        // CreateMetadataAccountV3Cpi::new(
        //     &cpi_program,
        //     cpi_accounts,
        //     args,
        // )
        // .invoke_signed(signer_seeds)?;

        Ok(())
    }
}