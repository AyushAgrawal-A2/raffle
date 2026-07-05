use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{constants::*, error::RaffleError, state::Raffle, RaffleStatus};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Reveal<'info> {
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
    #[account(owner = switchboard_on_demand::get_switchboard_on_demand_program_id())]
    pub randomness_account: UncheckedAccount<'info>,
}

pub fn handle_reveal(ctx: Context<Reveal>, _id: u64) -> Result<()> {
    require!(
        ctx.accounts.raffle.status == RaffleStatus::AwaitingRandomness,
        RaffleError::RandomnessAlreadyRequested
    );
    ctx.accounts.raffle.status = RaffleStatus::Drawn;

    require_keys_eq!(
        ctx.accounts.randomness_account.key(),
        ctx.accounts.raffle.randomness_account,
        RaffleError::WrongRandomnessAccount
    );
    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account.data.borrow())
            .map_err(|_| RaffleError::RandomnessAlreadyRequested)?;
    let clock = Clock::get()?;
    let revealed_value = randomness_data
        .get_value(clock.slot)
        .map_err(|_| RaffleError::RandomnessNotResolved)?;

    let winner_inder = u32::from_le_bytes(revealed_value[0..4].try_into().unwrap())
        % ctx.accounts.raffle.entrants.len() as u32;
    ctx.accounts.raffle.winner = Some(winner_inder);

    Ok(())
}
