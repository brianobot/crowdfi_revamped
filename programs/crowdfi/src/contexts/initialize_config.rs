use anchor_lang::prelude::*;

use crate::state::Config;


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>,
}


impl<'info> InitializeConfig<'info> {
    pub fn init(&mut self, seed: u64, max_duration: u64, max_amount: u64, bump: &InitializeConfigBumps) -> Result<()> {
        let fee = self.calculate_fee(max_amount, max_duration);

        self.config.set_inner( Config {
            admin: self.admin.key(),
            max_duration,
            max_amount,
            fee: fee,
            bump: bump.config,
            seed,
        });

        Ok(())
    }

    fn calculate_fee(&mut self, _max_amount: u64, _max_duration: u64) -> u16 {
        10
    }
}