use anchor_lang::prelude::*;

use crate::state::Config;


#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}


impl<'info> InitializeConfig<'info> {
    pub fn init(&mut self, max_duration: u64, max_amount: u64, bump: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner( Config {
            admin: self.admin.key(),
            max_duration,
            max_amount,
            fee: 10,
            bump: bump.config,
            seed: 0,
        });

        Ok(())
    }
}