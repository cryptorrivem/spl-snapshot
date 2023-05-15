use super::context::TokenTransferAccounts;
use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;

pub fn token_transfer(ctx: Context<TokenTransferAccounts>) -> Result<()> {
    let current = ctx.accounts.token_account.amount;
    msg!("Balance is {}", current);
    let diff = current - ctx.accounts.snapshot_account.amount;
    msg!("Increment was {}", diff);

    let cpi_program = ctx.accounts.token_account.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.token_account.to_account_info(),
        to: ctx.accounts.destination_account.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    anchor_spl::token::transfer(cpi_ctx, diff)
}
