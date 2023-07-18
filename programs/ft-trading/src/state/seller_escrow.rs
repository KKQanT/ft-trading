use anchor_lang::prelude::*;
use anchor_spl::{associated_token, token::TokenAccount};

#[account]

pub struct SellerEscrow {
    pub escrow_id: Pubkey,
    pub seller: Pubkey,
    pub token_address: Pubkey,
    pub price_per_token: u64,
    pub amount: u64,
}

impl SellerEscrow {
    pub const LEN: usize = 8 + 32 + 32 + 8;
    pub fn verify_token_account(
        &self, seller_account_key: &Pubkey, 
        token_account: &mut Box<Account<'_, TokenAccount>>
    ) -> bool {
        let expected_token_account = associated_token::get_associated_token_address(
            seller_account_key,
            &self.token_address
        );

        if expected_token_account == token_account.key() {
            true
        } else {
            false
        }

    }
}