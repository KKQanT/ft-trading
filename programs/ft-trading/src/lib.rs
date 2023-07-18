use anchor_lang::prelude::*;

pub mod intructions;
pub use intructions::*;

pub mod constant;
pub use constant::*;

pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

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
