#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("4JZUud5jkp1diBpkVqE1N7Ra62UEmyHYUhFqgbuT5bBZ");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
