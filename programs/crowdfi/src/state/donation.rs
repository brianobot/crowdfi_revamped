use anchor_lang::prelude::*;



#[account]
#[derive(InitSpace)]
pub struct Donation {
    pub authority: Pubkey,
    pub amount: u64,
    pub bump: u8,
}