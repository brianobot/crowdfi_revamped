use anchor_lang::error_code;


#[error_code]
pub enum CrowdfiError {
    #[msg("Campaign has not met it target")]
    CampaignTargetNotMet
}