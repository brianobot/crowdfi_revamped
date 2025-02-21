use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Config {
    pub admin: Pubkey,
    pub max_duration: u64, // the max duration campaigns under this config should run
    pub max_amount: u64, // the max amount campaingns under this config should process
    pub fee: u16, // the fee to be charged for campaigns under this config
    pub bump: u8,
    pub seed: u64,
}