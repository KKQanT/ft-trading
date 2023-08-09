use anchor_lang::prelude::*;
use anchor_spl::{token::TokenAccount, associated_token};

#[account]

pub struct WhitelistedNFT {
    pub token_address: Pubkey,
    pub last_claimed_epoch: u64,
}

impl  WhitelistedNFT {
    pub const LEN: usize = 8 + 32 + 8;

    pub fn verify_token_account(
        &self, 
        user_token_account: &Box<Account<'_, TokenAccount>>,
        owner_key: &Pubkey
     ) -> bool {
        
        let expected_token_account = associated_token::get_associated_token_address(
            owner_key, 
            &self.token_address
        );

        if expected_token_account.key() != user_token_account.key() {
            return false;
        }
        true
     }
}