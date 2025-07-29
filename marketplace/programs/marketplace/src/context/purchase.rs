use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::{MasterEditionAccount, Metadata, MetadataAccount}, 
    token::{close_account, mint_to, transfer_checked, CloseAccount, MintTo, TransferChecked}, 
    token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds = [b"treasury, marketplace.key().as_ref()"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

///Full disclosure: the following lines were generated with the aid of a LLM
impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        let fee_amount = (self.listing.price * self.marketplace.fee as u64) / 10000; // Asumiendo fee en basis points
        let seller_amount = self.listing.price - fee_amount;

        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, fee_amount)?;

        
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, seller_amount)?;

        Ok(())
    }

    pub fn send_nft(&mut self) -> Result<()> {
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
            to: self.taker_ata.to_account_info(),
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
