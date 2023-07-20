use anchor_lang::prelude::*;
use crate::{state::WhitelistedNFT, SUPER_AUTHORITY, DummyError, DividendVault, DividendVaultWallet};
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(epoch: u64)]
pub struct CreateDividendVault<'info> {
    #[account(
        init,
        seeds = [
            b"dividend_vault",
            epoch.to_le_bytes().as_ref()
        ],
        bump,
        payer = admin,
        space = DividendVault::LEN
    )]
    pub dividend_vault: Account<'info, DividendVault>,
    #[account(
        init,
        seeds = [
            b"dividend_vault_waller",
            dividend_vault.key().as_ref()
        ],
        bump,
        payer = admin,
        space = DividendVaultWallet::LEN
    )]
    pub dividend_vault_wallet: Account<'info, DividendVaultWallet>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn create_dividend_vault(
    ctx: Context<CreateDividendVault>,
    epoch: u64,
) -> Result<()> {

    let admin = &mut ctx.accounts.admin;
    let dividend_vault = &mut ctx.accounts.dividend_vault;

    let valid_admin = verify_super_auth(admin);
    if !valid_admin {
        msg!("fatal: cinvalid admin");
        return err!(DummyError::Error)
    }

    dividend_vault.epoch = epoch;
    dividend_vault.lamport_dividend_amount = 0;
    dividend_vault.total_share = 0;

    Ok(())
}

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

pub fn whitelist_nft(
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