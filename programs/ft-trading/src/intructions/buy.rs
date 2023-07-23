use anchor_lang::{prelude::*, system_program};
use anchor_spl::{token::{self, TokenAccount}, associated_token};

use crate::{SellerEscrow, DummyError, DividendVault, DividendVaultWallet, seller_escrow, PLATFORM_FEE};

#[derive(Accounts)]
#[instruction(
    escrow_id: Pubkey,
    token_address: Pubkey,
    amount: u64,
    seller_escrow_bump: u8,
    epoch: u64,
    dividend_vault_bump: u8,
    dividend_vault_wallet_bump: u8
)]
pub struct Buy<'info> {
    #[account(
        mut,
        seeds = [
            b"seller_escrow",
            seller.key().as_ref(),
            token_address.as_ref(),
            escrow_id.as_ref()
        ],
        bump = seller_escrow_bump,
    )]
    pub seller_escrow: Account<'info, SellerEscrow>,
    pub seller: AccountInfo<'info>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub escrow_token_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        seeds = [
            b"dividend_vault",
            epoch.to_le_bytes().as_ref()
        ],
        bump=dividend_vault_bump,
    )]
    pub dividend_vault: Account<'info, DividendVault>,
    #[account(
        mut,
        seeds = [
            b"dividend_vault_waller",
            dividend_vault.key().as_ref()
        ],
        bump= dividend_vault_wallet_bump
    )]
    pub dividend_vault_wallet: Account<'info, DividendVaultWallet>,
}

pub fn buy(
    ctx: Context<Buy>,
    escrow_id: Pubkey,
    token_address: Pubkey,
    amount: u64,
    seller_escrow_bump: u8,
    epoch: u64,
    dividend_vault_bump: u8,
    dividend_vault_wallet_bump: u8
) -> Result<()> {
    let seller_escrow = &mut ctx.accounts.seller_escrow;
    let seller = &mut ctx.accounts.seller;
    let buyer = &mut ctx.accounts.buyer;
    let buyer_token_account = &mut ctx.accounts.buyer_token_account;
    let escrow_token_account = &mut ctx.accounts.escrow_token_account;
    let token_program = &ctx.accounts.token_program;
    let dividend_vault = &mut ctx.accounts.dividend_vault;
    let dividend_vault_wallet = &mut ctx.accounts.dividend_vault_wallet;

    if seller_escrow.seller != seller.key() {
        msg!("invalid seller");
        return err!(DummyError::Error)
    }

    if seller_escrow.amount < amount {
        msg!("insufficient token");
        return err!(DummyError::Error)
    }

    let valid_buyer_token_account = verify_token_account(
        &token_address, 
        buyer_token_account,
        &buyer.key()
    );

    if !valid_buyer_token_account {
        msg!("fatal: invalid buyer token account");
        return err!(DummyError::Error)
    }

    let valid_escrow_token_account = seller_escrow.verify_token_account(
        &seller_escrow.key(),
        escrow_token_account
    );

    if !valid_escrow_token_account {
        msg!("fatal: invalid escrow token account");
        return err!(DummyError::Error)
    }

    let valid_epoch = dividend_vault.validate_epoch();
    if !valid_epoch {
        msg!("invalid epoch");
        return err!(DummyError::Error);
    }

    //calculate price to transfer
    let price = seller_escrow.get_price(amount);
    let platform_fee = (((PLATFORM_FEE as f64)/100.0) * (price as f64)) as u64;
    let seller_receive_amount = price - platform_fee;

    //transfer sol to seller
    let sol_to_user_cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        system_program::Transfer {
            from: buyer.to_account_info(),
            to: seller.clone()
        });
    
    system_program::transfer(
        sol_to_user_cpi_context, 
        seller_receive_amount
    )?;
    
    //transfer sol to us
    let sol_to_us_cpi_contecx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(), 
        system_program::Transfer {
            from: buyer.to_account_info(),
            to: dividend_vault_wallet.to_account_info()
        });
    
    system_program::transfer(
        sol_to_us_cpi_contecx, 
        platform_fee
    )?;

    //transfer stock to user

    let seller_key = seller.key();

    let seller_escrow_seeds = &[
        b"seller_escrow",
        seller_key.as_ref(),
        token_address.as_ref(),
        escrow_id.as_ref(),
        &[seller_escrow_bump]
    ];

    let seller_escrow_signers = [&seller_escrow_seeds[..]];

    let transfer_token_cpi_ctx = CpiContext::new_with_signer(
        token_program.to_account_info(),
        token::Transfer {
            from: escrow_token_account.to_account_info(),
            to: buyer_token_account.to_account_info(),
            authority: seller_escrow.to_account_info()
        },
        &seller_escrow_signers
    );

    token::transfer(transfer_token_cpi_ctx, amount)?;

    //accumulate fee we got    
    dividend_vault.lamport_dividend_amount += platform_fee;


    Ok(())
}

fn verify_token_account(
    token_address: &Pubkey,
    token_account: &mut Box<Account<'_, TokenAccount>>,
    owner: &Pubkey
) -> bool {
    let expected_token_account = associated_token::get_associated_token_address(
        owner, 
        token_address
    );

    if expected_token_account == token_account.key() {
        true
    } else {
        false
    }
}
