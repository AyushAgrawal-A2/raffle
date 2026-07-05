use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{constants::*, error::RaffleError, state::Raffle, RaffleStatus};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct RequestRandomness<'info> {
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

    /// CHECK: checked via owner check and parsed manually via RandomnessAccountData::parse
    #[account(owner = switchboard_on_demand::ON_DEMAND_MAINNET_PID)]
    pub randomness_account: UncheckedAccount<'info>,
}

pub fn handle_request_randomness(ctx: Context<RequestRandomness>, _id: u64) -> Result<()> {
    let clock = Clock::get()?;
    require!(
        clock.unix_timestamp >= ctx.accounts.raffle.end,
        RaffleError::RaffleEnded
    );
    require!(
        ctx.accounts.raffle.entrants.len() > 0,
        RaffleError::NoEntrant
    );
    require!(
        ctx.accounts.raffle.winner.is_none(),
        RaffleError::RandomnessAlreadyRequested
    );
    require!(
        ctx.accounts.raffle.status == RaffleStatus::Open,
        RaffleError::RandomnessAlreadyRequested
    );
    ctx.accounts.raffle.status = RaffleStatus::AwaitingRandomness;

    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account.data.borrow())
            .map_err(|_| RaffleError::RandomnessAlreadyRequested)?;
    require!(
        randomness_data.seed_slot == clock.slot - 1,
        RaffleError::RandomnessAlreadyRequested
    );
    ctx.accounts.raffle.randomness_account = ctx.accounts.randomness_account.key();

    Ok(())
}
