use anchor_lang::prelude::*;

#[account]
pub struct Raffle {
    pub id: u64,
    pub creator: Pubkey,
    pub start: i64,
    pub end: i64,
    pub entrants: Vec<Pubkey>,
    pub winner: Option<u32>,
    pub bump: u8,
}
impl Raffle {
    pub const PUBKEY_SPACE: usize = 32;
    pub const FIXED_SPACE: usize = 8 + 8 + Self::PUBKEY_SPACE + 8 + 8 + 4 + (4 + 1) + 1;
    pub fn space(entrants_len: usize) -> usize {
        Self::FIXED_SPACE + entrants_len * 32
    }
}
