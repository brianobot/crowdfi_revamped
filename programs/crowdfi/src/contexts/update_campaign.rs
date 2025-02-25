use anchor_lang::prelude::*;

use crate::state::Campaign;
use crate::error::CrowdfiError;


#[derive(Accounts)]
pub struct UpdateCampaign<'info> {
    #[account(
        mut,
        address = campaign.admin,
    )]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [b"campaign", campaign.title.as_bytes(), campaign.admin.as_ref()],
        bump = campaign.bump,
    )]
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}


impl<'info> UpdateCampaign<'info> {
    pub fn update(&mut self, description: Option<String>, url: Option<String>) -> Result<()> {
        let campaign = &mut self.campaign;

        if let Some(value) = description {
            require!(value.len() <= 250, CrowdfiError::CampaignDescriptionIsTooLong);
            campaign.description = value;
        }

        if let Some(value) = url {
            require!(value.len() <= 250, CrowdfiError::CampaignURLIsTooLong);
            campaign.url = value
        }
            
        Ok(())
    }
}