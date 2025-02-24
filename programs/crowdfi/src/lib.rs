use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

pub mod error;

declare_id!("14QSPRYYb9EyDHSXsqCNY4mcWKQ6dRycb8SNXeKHUcm4");


#[program]
pub mod crowdfi {
    use super::*;
    
    pub fn initialize_config(ctx: Context<InitializeConfig>, seed: u64, max_duration: u64, max_amount: u64) -> Result<()> {
        ctx.accounts.init(seed, max_duration, max_amount, &ctx.bumps)?;
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
        ctx.accounts.charge_fee(amount)?;
        ctx.accounts.mint_reward_token(amount)?;
        Ok(())
    }
    
    pub fn refund(ctx: Context<Refund>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_from_vault(amount)?;
        ctx.accounts.burn_reward_token(amount)?;
        Ok(())
    }

    pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
        // dotenv().ok();
        ctx.accounts.withdraw_from_vault()?;

        // check if the campaign is due and has met it target
        // if not call the refund donors function below
        // local_utils.get_token_accounts_for_mint
        // let helius_network = std::env::var("Network")
        //     .map_err(|_e| error!(error::CrowdfiError::CustomError))?;

        // let helius_api_key = std::env::var("HELIUS_API_KEY")
        //     .map_err(|_e| error!(error::CrowdfiError::CustomError))?;

        // let helius_url = format!("https://{}.helius-rpc.com/?api-key={}", helius_network, helius_api_key).as_str();

        // local_utils::get_token_accounts(helius_url, ctx.accounts.campaign_reward_mint.key());

        Ok(())
    }

}
