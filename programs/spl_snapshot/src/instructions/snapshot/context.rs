use crate::state::Snapshot;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SnapshotAccounts<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"snapshot", owner.key().as_ref()],
        bump,
        space = Snapshot::MAX_SIZE
    )]
    /// PDA for storing the balance of the owner account
    pub snapshot_account: Account<'info, Snapshot>,

    #[account(mut)]
    /// Account to snapshot its balance to determine the increment later
    pub owner: SystemAccount<'info>,

    #[account(mut)]
    /// Payer for the PDA temporary rent
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
