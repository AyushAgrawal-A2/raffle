use anchor_lang::prelude::*;

use crate::{constants::*, error::RaffleError, state::Raffle};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Enter<'info> {
    #[account(mut)]
    entrant: Signer<'info>,

    /// CHECK: checked via pda seeds and has_one constraint
    creator: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = creator,
        realloc = Raffle::space(raffle.entrants.len() + 1),
        realloc::payer = entrant,
        realloc::zero = false,
        seeds = [RAFFLE_SEED, creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = raffle.bump,
    )]
    raffle: Account<'info, Raffle>,

    system_program: Program<'info, System>,
}

pub fn handle_enter(ctx: Context<Enter>, _id: u64) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time >= ctx.accounts.raffle.start,
        RaffleError::RaffleNotStarted
    );
    require!(
        current_time < ctx.accounts.raffle.end,
        RaffleError::RaffleEnded
    );
    ctx.accounts
        .raffle
        .entrants
        .push(ctx.accounts.entrant.key());
    Ok(())
}
