use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::associated_token::AssociatedToken; 
use anchor_spl::token_interface::{
    Mint,
    mint_to, 
    MintTo,
    TokenAccount, 
    TokenInterface
};

use crate::state::{Campaign, Config, Donation};
use crate::error::CrowdfiError;


#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    // the person making the donation to the campaign
    pub signer: Signer<'info>, 
    #[account(
        mut, 
        address = config.admin
    )]
    // the admin that created the config for the campaign
    pub admin: SystemAccount<'info>, 
    #[account(
        address = campaign.admin
    )]
    pub campaign_admin: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.bump,
    )]
    pub config: Box<Account<'info, Config>>,
    #[account(
        mut,
        seeds = [b"campaign", campaign.title.as_bytes(), campaign_admin.key().as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Box<Account<'info, Campaign>>,
    #[account(
        mut,
        seeds = [b"campaign_vault", campaign.key().as_ref()],
        bump = campaign.vault_bump,
    )]
    // this would still be owned by my program
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        mint::decimals = 6,
        mint::authority = campaign.key(),
        seeds = [b"reward_mint", campaign.key().as_ref()],
        bump = campaign.reward_mint_bump,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + Donation::INIT_SPACE,
        seeds = [
            b"donation",
            signer.key().as_ref(),
            campaign.key().as_ref(),
        ],
        bump,
    )]
    pub donation_info: Box<Account<'info, Donation>>,
    #[account(
        init_if_needed,
        payer = signer,
        // the values passed to the authority and mint field
        // must be account field present in the account struct
        // anchor calls the .to_account_info() method on them behind the scene
        associated_token::authority = signer,
        associated_token::mint = reward_mint,
        associated_token::token_program = token_program,
    )]
    pub user_reward_ata: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}


impl<'info> Donate<'info> {
    pub fn transfer_to_vault(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, CrowdfiError::InvalidAmount);
        require!(self.campaign.is_completed != true, CrowdfiError::CampaignIsCompleted);

        let cpi_program = self.system_program.to_account_info();
        msg!("✅ Created CPI Program Variable");

        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };
        msg!("✅ Created CPI Accounts Variable");

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("✅ Created CPI Context Variable");

        // let _fee = amount - (self.config.fee as u64 * amount);

        transfer(cpi_ctx, amount)?;
        msg!("✅ Transferred Amount");

        // increment the current amount on the campaign data account
        let campaign = &mut self.campaign;
        // campaign.current_amount.checked_add(amount)?;
        campaign.current_amount += amount;

        Ok(())
    }

    pub fn charge_fee(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        msg!("✅ Created CPI Program Variable [Fee]");

        let cpi_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.admin.to_account_info(),
        };
        msg!("✅ Created CPI Accounts Variable [FEE]");

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let _fee = self.config.fee;

        let amount = (_fee as u64 / 100) * amount;

        transfer(cpi_ctx, amount)?;
        msg!("✅ Transfer Fee!");

        Ok(())
    }

    pub fn mint_reward_token(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        msg!("✅ Created CPI Program Variable [MINT]");

        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.campaign.to_account_info(),
        };

        let seeds = [
            b"campaign", 
            self.campaign.title.as_bytes(), 
            self.campaign_admin.to_account_info().key.as_ref(),
            &[self.campaign.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;
        
        Ok(())
    }

    pub fn update_donation_info(&mut self, amount: u64, bumps: &DonateBumps) -> Result<()> {
        let donation_info = &mut self.donation_info;

        donation_info.authority = self.signer.key();
        donation_info.campaign = self.campaign.key();
        donation_info.amount += amount;
        donation_info.bump = bumps.donation_info;

        Ok(())
    }
}