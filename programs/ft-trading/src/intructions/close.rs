use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::DummyError;
use crate::SellerEscrow;
use crate::verify_super_auth;

#[derive(Accounts)]
#[instruction(
    escrow_id: Pubkey,
    token_address: Pubkey,
    seller: Pubkey,
    bump: u8
)]
pub struct ForceCloseSell<'info> {
    #[account(
        mut,
        seeds = [
            b"seller_escrow",
            seller.as_ref(),
            token_address.as_ref(),
            escrow_id.as_ref()
        ],
        bump=bump,
        close=admin
    )]
    pub seller_escrow: Account<'info, SellerEscrow>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

pub fn handler(
    ctx: Context<ForceCloseSell>,
    _escrow_id: Pubkey,
    _token_address: Pubkey,
    _seller: Pubkey,
    _bump: u8
) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let valid_admin = verify_super_auth(admin);
    if !valid_admin {
        msg!("fatal: cinvalid admin");
        return err!(DummyError::Error);
    }

    Ok(())

}