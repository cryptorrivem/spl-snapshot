use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Snapshot {
    pub amount: u64,
    pub bump: u8,
}
impl Snapshot {
    pub const MAX_SIZE: usize = 8 + 8 + 1;
}
