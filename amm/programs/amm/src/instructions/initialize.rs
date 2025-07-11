use anchor_lang::prelude::*;
use anchor_spl::{}
use crate 


#[derive(Accounts)]

pub struct Initialize<'info> {
#[account(mut)]
pub initializer: Signer<'info>,

pub mint_x: Account<'info, Mint>,
pub mint_y: Account<'info, Mint>,

#[account(
    init,
    payer = initializer
)]






}