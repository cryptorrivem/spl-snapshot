use crate::state::Snapshot;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TokenSnapshotAccounts<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"snapshot", token_account.key().as_ref()],
        bump,
        space = Snapshot::MAX_SIZE
    )]
    /// PDA for storing the balance of the token account
    pub snapshot_account: Account<'info, Snapshot>,

    #[account()]
    /// CHECK: Account might not have been initialized. If it is, read its balance, else use 0
    /// Account to snapshot its balance to determine the increment later
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    /// Payer for the PDA temporary rent
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
