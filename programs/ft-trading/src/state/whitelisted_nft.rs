use anchor_lang::prelude::*;

#[account]

pub struct WhitelistedNFT {
    pub token_address: Pubkey,
    pub last_claim_ts: i64,
}

impl  WhitelistedNFT {
    pub const LEN: usize = 8 + 32 + 8;
}