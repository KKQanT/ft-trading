use anchor_lang::prelude::*;

use crate::{START_TS, EPOCH_DURATION};

#[account]
pub struct UserShareAccount {
    pub epoch: u64,
    pub n_share: u16,
    pub owner: Pubkey,
}

impl UserShareAccount {
    pub const LEN: usize = 8 + 8 + 2 + 32;

    pub fn validate_epoch(&self) -> bool {
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let current_epoch = ((now_ts - START_TS)/EPOCH_DURATION) as u64;
        if self.epoch == current_epoch {
            true 
        } else {
            msg!("epoch: {}", self.epoch.to_string().as_str());
            msg!("expected epoch: {}", current_epoch.to_string().as_str());
            false
        }
    }
}

#[account]
pub struct  DividendVault {
    pub epoch: u64,
    pub lamport_dividend_amount: u64,
    pub total_n_share: u16
}

impl DividendVault {
    pub const LEN: usize = 8 + 8 + 8 + 2;

    pub fn validate_epoch(&self) -> bool {
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let current_epoch = ((now_ts - START_TS)/EPOCH_DURATION) as u64;
        if self.epoch == current_epoch {
            true 
        } else {
            msg!("epoch: {}", self.epoch.to_string().as_str());
            msg!("expected epoch: {}", current_epoch.to_string().as_str());
            false
        }
    }
}

#[account]
pub struct DividendVaultWallet {}

impl  DividendVaultWallet {
    pub const LEN: usize = 8;
}
