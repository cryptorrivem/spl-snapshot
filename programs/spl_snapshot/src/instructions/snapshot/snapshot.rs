use super::context::SnapshotAccounts;
use anchor_lang::prelude::*;

pub fn snapshot(ctx: Context<SnapshotAccounts>) -> Result<()> {
    let account = &mut ctx.accounts.snapshot_account;
    account.amount = ctx.accounts.owner.lamports();
    msg!("Balance is {}", account.amount);
    account.bump = *ctx.bumps.get("snapshot_account").unwrap();

    Ok(())
}
