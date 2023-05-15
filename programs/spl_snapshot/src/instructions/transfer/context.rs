use crate::state::Snapshot;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [b"snapshot", owner.key().as_ref()],
        bump = snapshot_account.bump,
    )]
    /// PDA with the snapshot balance of the owner account
    pub snapshot_account: Account<'info, Snapshot>,

    #[account(mut,
        owner = system_program.key()
    )]
    /// Where to send the increment in lamports to
    pub destination: SystemAccount<'info>,

    #[account(mut)]
    /// Owner of the system account to approve the transfer of the balance increment
    pub owner: Signer<'info>,

    #[account(mut)]
    /// Refund the PDA temporary rent to the original payer
    pub payer: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}
