use anchor_lang::prelude::*;
// use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

declare_id!("LCoArfVL5kQiKe9kvoApLn3YFrE1MMrZ8Lj3Bmhumqg");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account(zero_copy)]
#[repr(C)]
pub struct ZeroCopyWithError {
    pub var1: u64,     // 8 bytes
    pub _padding: u64, // 8 bytes
    pub var2: u128,    // 16 bytes
}

#[derive(Accounts)]
pub struct SetPrice<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    // pub price_update: Account<'info, PriceUpdateV2>,
    // Add more accounts here
}
