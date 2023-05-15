use super::context::TransferAccounts;
use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;

pub fn transfer(ctx: Context<TransferAccounts>) -> Result<()> {
    let current = ctx.accounts.owner.lamports();
    msg!("Balance is {}", current);
    let diff = current - ctx.accounts.snapshot_account.amount;
    msg!("Increment was {}", diff);

    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    anchor_lang::system_program::transfer(cpi_ctx, diff)
}
