use anchor_lang::prelude::*;

#[account]

pub struct DividendVault {
    pub epoch: u64,
    pub claimable_ts: i64,
}

impl DividendVault {
    pub const LEN: usize = 8 + 8 + 8;
}

#[account]
pub struct EpochClaimed {}

impl EpochClaimed {
    pub const LEN: usize = 8;
}