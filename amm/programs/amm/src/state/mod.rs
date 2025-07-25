use anchor_lang::prelude::*;

#[account]
// Remember: InitSpace does not include the discriminator, 
// so we need to add it by ourselves.

#[derive(InitSpace)]

pub struct Config {
    pub seed: u64,
    pub authority: Option<Pubkey>,
    mint_x: Pubkey,
    mint_y: Pubkey,
    pub fee: u16,
    pub locked: bool,
    pub config_bump: u8,
    pub lp_bump: u8,
    
}
