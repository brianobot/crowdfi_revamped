use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::token::{burn, Burn};
use anchor_spl::token_interface::{Mint, TokenInterface, TokenAccount}; 


use crate::state::Campaign;


#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"campaign", campaign.title.as_bytes(), user.key().as_ref()],
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
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::authority = user,
        associated_token::mint = reward_mint,
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
            to: self.user.to_account_info(),
        };

        let seeds = [
            b"campaign_vault", 
            self.campaign.to_account_info().key.as_ref(),
            &[self.campaign.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn burn_reward_token(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Burn {
            mint: self.reward_mint.to_account_info(),
            from: self.user_reward_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)?;
        
        Ok(())
    }
}