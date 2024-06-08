use anchor_lang::prelude::*;

pub mod intructions;
pub use intructions::*;

pub mod constant;
pub use constant::*;

pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

declare_id!("S3TX6wejVn7n9d7CPtLySdBaaTgu8RdMCByrCDpEkR4");

#[program]
pub mod ft_trading {
    use super::*;

    pub fn create_dividend_vault(ctx: Context<CreateDividendVault>, epoch: u64) -> Result<()> {
        intructions::admin::create_dividend_vault(ctx, epoch)
    }

    pub fn add_whitelist_nft(ctx: Context<WhitelistNFT>, mint_address: Pubkey) -> Result<()> {
        intructions::admin::add_whitelist_nft(ctx, mint_address)
    }

    pub fn reset_whitelist_nft(
        ctx: Context<ResetWhitelistNFT>,
        mint_address: Pubkey,
        whitelist_nft_bump: u8,
    ) -> Result<()> {
        intructions::admin::reset_whitelist_nft(ctx, mint_address, whitelist_nft_bump)
    }

    pub fn sell(
        ctx: Context<Sell>,
        escrow_id: Pubkey,
        token_address: Pubkey,
        amount: u64,
        price_per_token: u64,
    ) -> Result<()> {
        intructions::sell::handler(ctx, escrow_id, token_address, amount, price_per_token)
    }

    pub fn buy(
        ctx: Context<Buy>,
        escrow_id: Pubkey,
        token_address: Pubkey,
        amount: u64,
        seller_escrow_bump: u8,
        epoch: u64,
        dividend_vault_bump: u8,
        dividend_vault_wallet_bump: u8,
    ) -> Result<()> {
        intructions::buy::handler(
            ctx,
            escrow_id,
            token_address,
            amount,
            seller_escrow_bump,
            epoch,
            dividend_vault_bump,
            dividend_vault_wallet_bump,
        )
    }

    pub fn create_share_account(ctx: Context<CreateShareAccount>, epoch: u64) -> Result<()> {
        intructions::claim::create_share_account(ctx, epoch)
    }

    pub fn claim_share(
        ctx: Context<ClaimShare>,
        epoch: u64,
        user_share_account_bump: u8,
        dividend_vault_bump: u8,
        whitelisted_nft_bump: u8,
    ) -> Result<()> {
        intructions::claim::claim_share(
            ctx,
            epoch,
            user_share_account_bump,
            dividend_vault_bump,
            whitelisted_nft_bump,
        )
    }

    pub fn claim_dividend(
        ctx: Context<ClaimDividend>,
        epoch: u64,
        user_share_account_bump: u8,
        dividend_vault_bump: u8,
        dividend_vault_wallet_bump: u8,
    ) -> Result<()> {
        intructions::claim::claim_dividend(
            ctx,
            epoch,
            user_share_account_bump,
            dividend_vault_bump,
            dividend_vault_wallet_bump,
        )
    }

    pub fn force_close_sell(
        ctx: Context<ForceCloseSell>,
        escrow_id: Pubkey,
        token_address: Pubkey,
        seller: Pubkey,
        bump: u8,
    ) -> Result<()> {
        intructions::close::handler(ctx, escrow_id, token_address, seller, bump)
    }
}
