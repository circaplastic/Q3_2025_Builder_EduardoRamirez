use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{close_account, transfer_checked, CloseAccount, TransferChecked}, 
    token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub maker_mint: InterfaceAccount<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.maker == maker.key(),
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let seeds = &[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        let seeds = &[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        close_account(cpi_ctx)?;

        Ok(())
    }
}