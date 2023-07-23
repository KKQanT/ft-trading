use anchor_lang::prelude::*;

use crate::{UserShareAccount, START_TS, EPOCH_DURATION};

#[derive(Accounts)]
pub struct CreateShareAccount<'info> {
    #[account(
        init,
        seeds = [
            b"user_share_account",
            owner.key().as_ref()
        ],
        bump,
        payer = owner,
        space = UserShareAccount::LEN
    )]
    user_share_account: Account<'info, UserShareAccount>,
    #[account(mut)]
    owner: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub fn create_share_account(
    ctx: Context<CreateShareAccount>
) -> Result<()> {
    let user_share_account = &mut ctx.accounts.user_share_account;
    let owner = &ctx.accounts.owner; 

    let now_ts = Clock::get().unwrap().unix_timestamp;
    let current_epoch = ((now_ts - START_TS)/EPOCH_DURATION) as u64;
    
    user_share_account.epoch = current_epoch;
    user_share_account.reward_share = 0;
    user_share_account.owner = owner.key();
    
    Ok(())
}

