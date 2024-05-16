use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use pyth_sdk_solana::state::{
    load_price_account, AccountType, PriceStatus, PriceType, Rational, SolanaPriceAccount, MAGIC,
    VERSION_2,
};

declare_id!("BjLKu19Kbqnc43dSDD7aKSrTxahskhkAsKh85Pmmewr2");
const N: usize = 32; // Example value for N, adjust as needed

#[program]
pub mod pyth_stub {
    use super::*;

    pub fn write_pyth_price(
        ctx: Context<Write>,
        price: i64,
        expo: i32,
        slot: i64,
        timestamp_sec: i64,
    ) -> ProgramResult {
        let account_data = &mut ctx.accounts.target.try_borrow_mut_data()?;
        let mut def = SolanaPriceAccount::default();
        let exist_price_data = load_price_account(&account_data).unwrap_or(&mut def);

        let mut price_data: SolanaPriceAccount = unsafe { std::mem::zeroed() };

        // // set defaults
        price_data.ptype = PriceType::Price;
        price_data.magic = MAGIC;
        price_data.ver = VERSION_2;
        price_data.atype = AccountType::Price as u32;

        price_data.valid_slot = slot.try_into().unwrap_or(exist_price_data.valid_slot);
        price_data.agg.price = price;
        price_data.expo = expo;
        price_data.agg.status = PriceStatus::Trading;
        price_data.timestamp = timestamp_sec;

        // set ema to price
        price_data.ema_price = Rational {
            val: price,
            numer: 0,
            denom: 0,
        };

        account_data.copy_from_slice(unsafe {
            &std::mem::transmute::<
                SolanaPriceAccount,
                [u8; std::mem::size_of::<SolanaPriceAccount>()],
            >(price_data)
        });

        msg!("Price written to account");
        // // check price feed is valid
        let price_account =
            load_price_account::<N, SolanaPriceAccount>(account_data).map_err(|e| {
                msg!("Error loading price pyth feed: {:?}", e);
                error!(OracleError::InvalidAccountData)
            });

        // let feed = price_account.to_price_feed(ctx.accounts.target.key);
        // msg!("price feed is {}", feed.get_price_unchecked().price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Write<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: just a mock program
    #[account(init_if_needed, payer = signer, space = std::mem::size_of::<SolanaPriceAccount>())]
    pub target: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum OracleError {
    #[msg("Existing account data cannot be parsed")]
    BadAccountData,
    #[msg("The pyth account could not be parsed")]
    InvalidAccountData,
}
