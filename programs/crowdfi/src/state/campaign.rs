use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub admin: Pubkey,
    #[max_len(250)]
    pub title: String,
    #[max_len(250)]
    pub description: String,
    #[max_len(250)]
    pub url: String,
    pub start_timestamp: u64, // the max duration campaigns under this config should run
    pub end_timestamp: u64,
    pub target_amount: u64, // the max amount campaingns under this config should process
    pub current_amount: u64, // the max amount campaingns under this config should process
    pub bump: u8,
    pub vault_bump: u8,
    pub reward_mint_bump: u8,
}