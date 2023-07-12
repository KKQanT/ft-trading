use anchor_lang::prelude::*;

#[account]

pub struct WhitelistedNFT {
    pub last_claimed_epoch: u64,
}

impl  WhitelistedNFT {
    pub const LEN: usize = 8 + 8;
}