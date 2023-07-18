use anchor_lang::prelude::*;
use anchor_spl::{token};

use crate::{SellerEscrow, DummyError};

#[derive(Accounts)]
#[instruction(
    escrow_id: Pubkey,
    token_address: Pubkey,
    amount: u64,
    price_per_token: u64,
)]
pub struct Sell<'info> {
    #[account(
        init,
        seeds = [
            b"seller_escrow",
            seller.key().as_ref(),
            token_address.as_ref(),
            escrow_id.as_ref()
        ],
        bump,
        payer=seller,
        space=SellerEscrow::LEN
    )]
    pub seller_escrow: Account<'info, SellerEscrow>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub seller_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

pub fn handler(
    ctx: Context<Sell>,
    escrow_id: Pubkey,
    token_address: Pubkey,
    amount: u64,
    price_per_token: u64,
) -> Result<()> {
    let seller_escrow = &mut ctx.accounts.seller_escrow;
    let seller = &ctx.accounts.seller;
    let seller_token_account = &mut ctx.accounts.seller_token_account;
    let escrow_token_account = &mut ctx.accounts.escrow_token_account;
    
    seller_escrow.token_address = token_address;

    let valid_escrow_token_account = seller_escrow.verify_token_account(
        &seller_escrow.key(),
        escrow_token_account
    );

    if seller_token_account.amount < amount {
        msg!("fatal: insufficient token");
        return err!(DummyError::Error)
    }

    if !valid_escrow_token_account {
        msg!("fatal: invalid escrow token account");
        return err!(DummyError::Error)
    }

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        token::Transfer {
            from: seller_token_account.to_account_info(),
            to: escrow_token_account.to_account_info(),
            authority: seller.to_account_info()
        },
    );

    token::transfer(cpi_ctx, amount)?;

    seller_escrow.escrow_id = escrow_id;
    seller_escrow.seller = seller.key();
    seller_escrow.price_per_token = price_per_token;
    seller_escrow.amount = amount;

    Ok(())
}