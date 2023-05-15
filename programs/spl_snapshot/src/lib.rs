use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("En9fSSnkmcgHoKx4wBjRf8RbGvL352f5zFfDu32UPN3m");

#[program]
pub mod spl_snapshot {
    use super::*;

    pub fn snapshot(ctx: Context<SnapshotAccounts>) -> Result<()> {
        instructions::snapshot(ctx)
    }

    pub fn transfer(ctx: Context<TransferAccounts>) -> Result<()> {
        instructions::transfer(ctx)
    }

    pub fn token_snapshot(ctx: Context<TokenSnapshotAccounts>) -> Result<()> {
        instructions::token_snapshot(ctx)
    }

    pub fn token_transfer(ctx: Context<TokenTransferAccounts>) -> Result<()> {
        instructions::token_transfer(ctx)
    }
}
