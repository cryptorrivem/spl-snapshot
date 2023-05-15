use crate::state::Snapshot;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct TokenTransferAccounts<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [b"snapshot", token_account.key().as_ref()],
        bump = snapshot_account.bump,
    )]
    /// PDA with the snapshot balance of the token account
    pub snapshot_account: Account<'info, Snapshot>,

    #[account(mut, has_one = owner)]
    /// Account to read its updated balance and transfer the increment since the snapshot
    pub token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_account.mint == destination_account.mint
    )]
    /// Where to send the increment in tokens to. Mint should match to token account's
    pub destination_account: Account<'info, TokenAccount>,

    #[account(mut)]
    /// Owner of the token account to approve the transfer of the balance increment
    pub owner: Signer<'info>,

    #[account(mut)]
    /// Refund the PDA temporary rent to the original payer
    pub payer: SystemAccount<'info>,

    pub token_program: Program<'info, Token>,
}
