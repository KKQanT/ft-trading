use anchor_lang::prelude::*;

pub mod intructions;
pub use intructions::*;

pub mod state;
pub use state::*;

declare_id!("SETaVQKfUNLS2xUR61uauMTptRdKyaKHfPYjDvHAnxv");

#[program]
pub mod ft_trading {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
