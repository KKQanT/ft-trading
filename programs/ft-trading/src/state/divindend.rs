use anchor_lang::prelude::*;

#[account]
pub struct UserDividendEpoch {
    pub epoch: u64,
    pub reward_share: u64,
}

impl UserDividendEpoch {
    pub const LEN: usize = 8 + 8 + 8;
}

#[account]
pub struct  DividendVault {
    pub epoch: u64,
    pub lamport_dividend_amount: u64,
    pub total_share: u64
}

impl DividendVault {
    pub const LEN: usize = 8 + 8*3;
}

#[account]
pub struct DividendVaultWallet {}

impl  DividendVaultWallet {
    pub const LEN: usize = 8;
}
