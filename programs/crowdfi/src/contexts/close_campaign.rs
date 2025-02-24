use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{metadata::mpl_token_metadata, token_interface::{Mint, TokenInterface}};

use crate::state::{Campaign, Config};
use crate::error::CrowdfiError;

#[derive(Accounts)]
pub struct CloseCampaign<'info> {
    #[account(
        mut,
        address = campaign.admin,
    )]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [b"campaign", campaign.title.as_bytes(), campaign.admin.as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(
        mut,
        seeds = [b"campaign_vault", campaign.key().as_ref()],
        bump = campaign.vault_bump,
    )]
    // this would still be owned by my program
    pub campaign_vault: SystemAccount<'info>,
    #[account(
        mut,
        mint::decimals = 6,
        mint::authority = campaign,
        seeds = [b"reward_mint", campaign.key().as_ref()],
        bump = campaign.reward_mint_bump,
    )]
    pub campaign_reward_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> CloseCampaign<'info> {
    pub fn withdraw_from_vault(&mut self) -> Result<()> {
        // if the campaign is compelted, it has already been closed before
        require!(self.campaign.is_completed != true, CrowdfiError::CampaignIsCompleted);

        let has_met_target_amount = self.campaign.current_amount >= self.campaign.target_amount;

        if has_met_target_amount {
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.campaign_vault.to_account_info(),
                to: self.signer.to_account_info(),
            };

            let seeds = [
                b"campaign_vault", 
                self.campaign.to_account_info().key.as_ref(),
                &[self.campaign.vault_bump],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, self.campaign.current_amount)?;
        }
        

        Ok(())
    }

    pub fn mark_as_is_completed(&mut self) -> Result<()> {
        let campaign = &mut self.campaign;

        campaign.is_completed = true;

        Ok(())
    }
}