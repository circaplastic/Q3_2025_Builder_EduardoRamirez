/*Escrow program succesfully deployed to devnet
Signature: 5cr9ptATxkg5TvnXJQEP3jYXvT3mZ6dZ2A5yBnSLp7idF3GzSiwpz3eF5YFrb11AUhyE2TW6d4mExfdo3RiWBzJb
Deploy success
*/

#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("4JZUud5jkp1diBpkVqE1N7Ra62UEmyHYUhFqgbuT5bBZ");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }

    pub fn take(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}

