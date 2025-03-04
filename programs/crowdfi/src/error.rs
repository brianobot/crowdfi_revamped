use anchor_lang::error_code;


#[error_code]
pub enum CrowdfiError {
    #[msg("Custom Error")]
    CustomError,
    #[msg("Invalid Amount Value")]
    InvalidAmount,
    #[msg("Campaign has not met it target")]
    CampaignTargetNotMet,
    #[msg("Campaign has been completed already")]
    CampaignIsCompleted,
    #[msg("Campaign Title is Too Long")]
    CampaignTitleIsTooLong,
    #[msg("Campaign Description is Too Long")]
    CampaignDescriptionIsTooLong,
    #[msg("Campaign URL is Too Long")]
    CampaignURLIsTooLong,
}