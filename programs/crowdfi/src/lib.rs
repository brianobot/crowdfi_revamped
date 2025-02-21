use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;


declare_id!("14QSPRYYb9EyDHSXsqCNY4mcWKQ6dRycb8SNXeKHUcm4");


#[program]
pub mod crowdfi {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, max_duration: u64, max_amount: u64) -> Result<()> {
        ctx.accounts.init(max_duration, max_amount, &ctx.bumps)?;
        Ok(())
    }
    
    pub fn create_campaign(ctx: Context<CreateCampaign>, title: String, description: String, url: String, target_amount: u64, start_timestamp: u64, end_timestamp: u64) -> Result<()> {
        ctx.accounts.init(title, description, url, target_amount, start_timestamp, end_timestamp, &ctx.bumps)?;
        Ok(())
    }
    
    pub fn update_campaign(ctx: Context<UpdateCampaign>, description: Option<String>, url: Option<String>) -> Result<()> {
        ctx.accounts.update(description, url)?;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        ctx.accounts.transfer_to_vault(amount)?;
        ctx.accounts.mint_reward_token(amount)?;
        Ok(())
    }
    
    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_from_vault(amount)?;
        ctx.accounts.burn_reward_token(amount)?;
        Ok(())
    }

    // pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
    //     ctx.accounts.close_campaign()?;
    //     Ok(())
    // }

}
