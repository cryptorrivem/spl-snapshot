use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
use std::mem::size_of;

declare_id!("En9fSSnkmcgHoKx4wBjRf8RbGvL352f5zFfDu32UPN3m");

#[program]
pub mod spl_snapshot {
    use super::*;

    pub fn snapshot(ctx: Context<SnapshotAccounts>) -> Result<()> {
        let account = &mut ctx.accounts.snapshot_account;
        account.amount = ctx.accounts.token_account.amount;
        account.bump = *ctx.bumps.get("snapshot_account").unwrap();

        Ok(())
    }

    pub fn transfer(ctx: Context<TransferAccounts>) -> Result<()> {
        let current = ctx.accounts.token_account.amount;
        let diff = current - ctx.accounts.snapshot_account.amount;

        let cpi_program = ctx.accounts.token_account.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.destination_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        anchor_spl::token::transfer(cpi_ctx, diff)
    }
}

#[derive(Accounts)]
pub struct SnapshotAccounts<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [b"snapshot", token_account.key().as_ref()],
        bump,
        space = size_of::<Snapshot>()
    )]
    pub snapshot_account: Account<'info, Snapshot>,

    #[account(has_one = owner)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"snapshot", token_account.key().as_ref()],
        bump = snapshot_account.bump,
    )]
    pub snapshot_account: Account<'info, Snapshot>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_account.mint == destination_account.mint
    )]
    pub destination_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct Snapshot {
    amount: u64,
    bump: u8,
}
