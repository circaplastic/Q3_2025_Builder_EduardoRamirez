//Signature: QdQFBDQS7cR7etQpDopLeZbGcJahrcd3LwraEhMT9wW3Um36puBvm9ZdzoZVbP6ouA86vAhxhrka5PZTKxiYBQf
//Deploy success

#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("61z328xZYQ4ou2M22KEY8r9GHJja6bcg8oDor627SGXz");

pub mod instructions;
pub mod state;
pub use instructions::*;
pub use state::*;

#[program]

pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }
}