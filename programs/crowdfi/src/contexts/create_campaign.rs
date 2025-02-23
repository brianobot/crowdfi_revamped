use anchor_lang::prelude::*;
use anchor_spl::{metadata::mpl_token_metadata, token_interface::{Mint, TokenInterface}};

use crate::state::{Campaign, Config};


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
    pub campaign_vault: SystemAccount<'info>,
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
    pub campaign_reward_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> CreateCampaign<'info> {
    pub fn init(&mut self, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64, bumps: &CreateCampaignBumps) -> Result<()> {
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
            vault_bump: bumps.campaign_vault,
            reward_mint_bump: bumps.campaign_reward_mint,
        });
        Ok(())
    }

    pub fn create_mint_metadata(&mut self) -> Result<()> {
        let seeds = [
            b"metadata",
            self.campaign_reward_mint.to_account_info().key.as_ref(),
            mpl_token_metadata::ID.as_ref(),
        ];

        let (_metadata_key, _bump) = Pubkey::find_program_address(&seeds, &mpl_token_metadata::ID);

        // mpl_token_metadata::accounts::CreateMetadataAccountsV3

        Ok(())
    }
}