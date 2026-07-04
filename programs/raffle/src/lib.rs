pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Aj79PCKC7FAiGoKTCw6AYdhirh2YHHkoAawYuVKshuDi");

#[program]
pub mod raffle {
    use super::*;

    pub fn create(ctx: Context<Create>, id: u64, start: i64, end: i64, amount: u64) -> Result<()> {
        crate::instructions::create::handle_create(ctx, id, start, end, amount)
    }

    pub fn enter(ctx: Context<Enter>, id: u64) -> Result<()> {
        crate::instructions::enter::handle_enter(ctx, id)
    }

    pub fn draw(ctx: Context<Draw>, id: u64) -> Result<()> {
        crate::instructions::draw::handle_draw(ctx, id)
    }

    pub fn claim(ctx: Context<Claim>, id: u64) -> Result<()> {
        crate::instructions::claim::handle_claim(ctx, id)
    }
}
