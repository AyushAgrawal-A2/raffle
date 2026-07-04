use anchor_lang::prelude::*;

use crate::{constants::*, error::RaffleError, state::Raffle};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Draw<'info> {
    signer: Signer<'info>,

    /// CHECK: checked via pda seeds and has_one constraint
    creator: UncheckedAccount<'info>,

    #[account(
        mut,
        has_one = creator,
        seeds = [RAFFLE_SEED, creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = raffle.bump,
    )]
    raffle: Account<'info, Raffle>,
}

pub fn handle_draw(ctx: Context<Draw>, _id: u64) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    require!(
        current_time >= ctx.accounts.raffle.end,
        RaffleError::RaffleEnded
    );
    require!(
        ctx.accounts.raffle.entrants.len() > 0,
        RaffleError::NoEntrant
    );
    require!(
        ctx.accounts.raffle.winner.is_none(),
        RaffleError::RaffleAlreadyDrawn
    );
    let winner_index = Clock::get()?.unix_timestamp as usize % ctx.accounts.raffle.entrants.len();
    ctx.accounts.raffle.winner = Some(winner_index as u32);
    Ok(())
}
