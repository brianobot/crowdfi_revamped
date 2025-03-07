use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

pub mod error;

declare_id!("GMvQJy82KJryDR6nk2WEti4oYTLJjFKH2EbsdSjtgSMu");


#[program]
pub mod crowdfi {
    use super::*;
    
    pub fn initialize_config(ctx: Context<InitializeConfig>, seed: u64, max_duration: u64, max_amount: u64) -> Result<()> {
        ctx.accounts.init(seed, max_duration, max_amount, &ctx.bumps)?;
        Ok(())
    }
    
    pub fn create_campaign(ctx: Context<CreateCampaign>, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64) -> Result<()> {
        ctx.accounts.init(title.clone(), description, url, target_amount, start_timestamp, end_timestamp, &ctx.bumps)?;
        // ctx.accounts.init_mint_metadata(title)?;
        Ok(())
    }
    
    pub fn update_campaign(ctx: Context<UpdateCampaign>, description: Option<String>, url: Option<String>) -> Result<()> {
        ctx.accounts.update(description, url)?;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_to_vault(amount)?;
        ctx.accounts.charge_fee(amount)?;
        ctx.accounts.mint_reward_token(amount)?;
        ctx.accounts.update_donation_info(amount, &ctx.bumps)?;
        Ok(())
    }
    
    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_from_vault(amount)?;
        ctx.accounts.burn_reward_token(amount)?;
        ctx.accounts.update_donation_info(amount)?;
        Ok(())
    }

    pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
        ctx.accounts.withdraw_from_vault()?;
        ctx.accounts.mark_as_is_completed()?;
        Ok(())
    }

}
