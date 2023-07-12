use anchor_lang::prelude::*;

#[account]

pub struct SellerEscrow {
    pub owner: Pubkey,
    pub token_address: Pubkey,
    pub price: u64,
}

impl SellerEscrow {
    pub const LEN: usize = 8 + 32 + 32 + 8;
}