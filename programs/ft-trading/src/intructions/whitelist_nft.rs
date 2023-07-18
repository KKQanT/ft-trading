use anchor_lang::prelude::*;
use crate::{state::WhitelistedNFT, SUPER_AUTHORITY, DummyError};
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(
    mint_address: Pubkey,
)]

pub struct  WhitelistNFT<'info> {
    #[account(
        init,
        seeds = [
            b"whitelist_nft",
            mint_address.as_ref()
        ],
        bump,
        payer = admin,
        space = WhitelistedNFT::LEN
    )]
    pub whitelist_nft: Account<'info, WhitelistedNFT>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn handler(
    ctx: Context<WhitelistNFT>,
    mint_address: Pubkey
) -> Result<()> {
    let whitelist_nft = &mut ctx.accounts.whitelist_nft;
    let admin = &ctx.accounts.admin;
    let valid_admin = verify_super_auth(admin);
    if !valid_admin {
        msg!("fatal: cinvalid admin");
        return err!(DummyError::Error)
    }

    whitelist_nft.token_address = mint_address;

    Ok(())
}

fn verify_super_auth(super_auth: &Signer) -> bool {
    let expected_account = Pubkey::from_str(SUPER_AUTHORITY).unwrap();
    if super_auth.key() == expected_account {
        true
    } else {
        false
    }
}