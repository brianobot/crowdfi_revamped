use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::associated_token::AssociatedToken; 
use anchor_spl::token::{mint_to, MintTo};
use anchor_spl::token_interface::{
    Mint, 
    TokenAccount, 
    TokenInterface
};

use crate::state::{Campaign, Config};


#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(address = config.admin)]
    pub admin: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
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
        init_if_needed,
        payer = user,
        associated_token::authority = user,
        associated_token::mint = reward_mint,
    )]
    pub user_reward_ata: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> Donate<'info> {
    pub fn transfer_to_vault(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.campaign_vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // let _fee = amount - (self.config.fee as u64 * amount);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn charge_fee(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.admin.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let _fee = self.config.fee;

        let amount = 1u64;

        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn mint_reward_token(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.campaign.to_account_info(),
        };

        let seeds = [
            b"campaign", 
            self.campaign.title.as_bytes(), 
            self.user.to_account_info().key.as_ref(),
            &[self.campaign.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;
        
        Ok(())
    }
}