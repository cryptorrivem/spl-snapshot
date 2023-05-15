use super::context::TokenSnapshotAccounts;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

pub fn token_snapshot(ctx: Context<TokenSnapshotAccounts>) -> Result<()> {
    let account = &mut ctx.accounts.snapshot_account;
    account.amount = match Account::<TokenAccount>::try_from_unchecked(&ctx.accounts.token_account)
    {
        Ok(acc) => acc.amount,
        Err(_) => 0,
    };
    msg!("Balance is {}", account.amount);
    account.bump = *ctx.bumps.get("snapshot_account").unwrap();

    Ok(())
}
