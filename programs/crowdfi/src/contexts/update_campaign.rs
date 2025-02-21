use anchor_lang::prelude::*;

use crate::state::Campaign;


#[derive(Accounts)]
pub struct UpdateCampaign<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"campaign", campaign.title.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}


impl<'info> UpdateCampaign<'info> {
    pub fn update(&mut self, description: Option<String>, url: Option<String>) -> Result<()> {
        let campaign = &mut self.campaign;

        if let Some(value) = description {
            campaign.description = value;
        }

        if let Some(value) = url {
            campaign.url = value
        }
            
        Ok(())
    }
}