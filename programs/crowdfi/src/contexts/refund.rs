use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount, burn, Burn}; 


use crate::state::{Campaign, Donation};


#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut, 
        seeds = [b"campaign", campaign.title.as_bytes(), campaign.admin.as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Box<Account<'info, Campaign>>,
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
    #[account(
        mut,
        seeds = [
            b"donation",
            signer.key().as_ref(),
            campaign.key().as_ref(),
        ],
        bump = donation_info.bump,
    )]
    pub donation_info: Box<Account<'info, Donation>>,
    #[account(
        mut,
        associated_token::authority = signer,
        associated_token::mint = campaign_reward_mint,
    )]
    pub user_reward_ata: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}


impl<'info> Refund<'info> {
    pub fn withdraw_from_vault(&mut self, amount: u64) -> Result<()> {
        // let user_reward_ata  = &mut self.user_reward_ata;
        // require!(user_reward_ata.amount <= amount);

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.campaign_vault.to_account_info(),
            to: self.signer.to_account_info(),
        };

        let seeds = [
            b"campaign_vault", 
            self.campaign.to_account_info().key.as_ref(),
            &[self.campaign.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;
        
        // increment the current amount on the campaign data account
        let campaign = &mut self.campaign;
        // campaign.current_amount.checked_add(amount)?;
        campaign.current_amount -= amount;
        
        Ok(())
    }

    pub fn burn_reward_token(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            mint: self.campaign_reward_mint.to_account_info(),
            from: self.user_reward_ata.to_account_info(),
            authority: self.signer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)?;
        
        Ok(())
    }

    pub fn update_donation_info(&mut self, amount: u64) -> Result<()> {
        let donation_info = &mut self.donation_info;

        donation_info.amount -= amount;

        Ok(())
    }
}