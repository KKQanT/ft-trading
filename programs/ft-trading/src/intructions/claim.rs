use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::{UserShareAccount, START_TS, EPOCH_DURATION, DividendVault, WhitelistedNFT, instruction, DividendVaultWallet};
use crate::DummyError;

#[derive(Accounts)]
#[instruction(epoch: u64)]
pub struct CreateShareAccount<'info> {
    #[account(
        init,
        seeds = [
            b"user_share_account",
            epoch.to_le_bytes().as_ref(),
            owner.key().as_ref()
        ],
        bump,
        payer = owner,
        space = UserShareAccount::LEN
    )]
    pub user_share_account: Account<'info, UserShareAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn create_share_account(
    ctx: Context<CreateShareAccount>,
    epoch: u64
) -> Result<()> {
    let user_share_account = &mut ctx.accounts.user_share_account;
    let owner = &mut ctx.accounts.owner; 

    let now_ts = Clock::get().unwrap().unix_timestamp;
    let current_epoch = ((now_ts - START_TS)/EPOCH_DURATION) as u64;

    if current_epoch != epoch {
        msg!("invalid epoch");
        msg!("epoch: {}", epoch);
        msg!("current epoch: {}", current_epoch)
    }
    
    user_share_account.epoch = current_epoch;
    user_share_account.reward_share = 0;
    user_share_account.owner = owner.key();
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    epoch: u64, 
    user_share_account_bump: u8,
    dividend_vault_bump: u8,
    whitelisted_nft_bump: u8
)]
pub struct ClaimShare<'info> {
    #[account(
        mut,
        seeds = [
            b"user_share_account",
            epoch.to_le_bytes().as_ref(),
            owner.key().as_ref()
        ],
        bump = user_share_account_bump
    )]
    pub user_share_account: Account<'info, UserShareAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
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
            b"whitelist_nft",
            whitelisted_nft.token_address.as_ref()
        ],
        bump = whitelisted_nft_bump
    )]
    pub whitelisted_nft: Account<'info, WhitelistedNFT>,
    pub user_token_account: Box<Account<'info, TokenAccount>>,
}

pub fn claim_share(
    ctx: Context<ClaimShare>,
    _epoch: u64, 
    _user_share_account_bump: u8,
    _dividend_vault_bump: u8,
    _whitelisted_nft_bump: u8
) -> Result<()> {

    let user_share_account = &mut ctx.accounts.user_share_account;
    let owner = &mut ctx.accounts.owner;
    let dividend_vault = &mut ctx.accounts.dividend_vault;
    let whitelisted_nft = &mut ctx.accounts.whitelisted_nft;
    let user_token_account = &ctx.accounts.user_token_account;
    let now_ts = Clock::get().unwrap().unix_timestamp;

    if user_share_account.owner != owner.key() {
        msg!("invalid user share account onwer");
        return err!(DummyError::Error);
    }

    let valid_epoch_1 = user_share_account.validate_epoch();
    let valid_epoch_2 = dividend_vault.validate_epoch();

    if !(valid_epoch_1 && valid_epoch_2) {
        msg!("invalid epoch");
        return  err!(DummyError::Error);
    }
    
    let valid_token_account = whitelisted_nft.verify_token_account(
        user_token_account, 
        &owner.key()
    );

    if !valid_token_account {
        msg!("invalid nft token account");
        return err!(DummyError::Error);
    }

    if user_token_account.amount != 1 {
        msg!("you dont have nft");
        return err!(DummyError::Error);
    }

    //save data
    let reward_share_received = (now_ts - whitelisted_nft.last_claim_ts) as u64;

    user_share_account.reward_share += reward_share_received;
    dividend_vault.total_share += reward_share_received;
    whitelisted_nft.last_claim_ts = now_ts;

    Ok(())
}

#[derive(Accounts)]
#[instruction(
    epoch: u64,
    user_share_account_bump: u8,
    dividend_vault_bump: u8,
    dividend_vault_wallet_bump: u8,
)]
pub struct ClaimDividend<'info> {
    #[account(
        mut,
        seeds = [
            b"user_share_account",
            epoch.to_le_bytes().as_ref(),
            owner.key().as_ref()
        ],
        bump = user_share_account_bump,
        close = owner
    )]
    pub user_share_account: Account<'info, UserShareAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
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

pub fn claim_dividend(
    ctx: Context<ClaimDividend>,
    epoch: u64,
    _user_share_account_bump: u8,
    _dividend_vault_bump: u8,
    _dividend_vault_wallet_bump: u8
) -> Result<()> {
    let user_share_account = &mut ctx.accounts.user_share_account;
    let owner = &mut ctx.accounts.owner;
    let dividend_vault = &ctx.accounts.dividend_vault;
    let dividend_vault_wallet = &mut ctx.accounts.dividend_vault_wallet;

    if user_share_account.owner != owner.key() {
        msg!("user_share_account.owner != owner.key()");
        return err!(DummyError::Error);
    }

    if user_share_account.epoch != epoch {
        msg!("user_share_account.epoch != epoch");
        return err!(DummyError::Error);
    }

    if user_share_account.epoch != dividend_vault.epoch {
        msg!("user_share_account.epoch != dividend_vault.epoch");
        return err!(DummyError::Error);
    }

    let now_ts = Clock::get().unwrap().unix_timestamp;
    let current_epoch = ((now_ts - START_TS)/EPOCH_DURATION) as u64;

    if user_share_account.epoch >= current_epoch {
        msg!("can't claim now");
        return err!(DummyError::Error);
    }

    let reward_amount = 
    ((user_share_account.reward_share as f64) 
    / (dividend_vault.total_share as f64) 
    * (dividend_vault.lamport_dividend_amount as f64)) as u64;

    let src = &mut dividend_vault_wallet.to_account_info();
    **src.try_borrow_mut_lamports()? = src
    .lamports()
    .checked_sub(reward_amount)
    .ok_or(ProgramError::InvalidArgument)?;

    let dst = &mut owner.to_account_info();
    **dst.try_borrow_mut_lamports()? = dst
    .lamports()
    .checked_add(reward_amount)
    .ok_or(ProgramError::InvalidArgument)?;

    Ok(())
}