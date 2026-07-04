use anchor_lang::prelude::*;

#[error_code]
pub enum RaffleError {
    #[msg("Invalid arguments")]
    InvalidArguments,
    #[msg("Raffle not started")]
    RaffleNotStarted,
    #[msg("Raffle ended")]
    RaffleEnded,
    #[msg("No entrant")]
    NoEntrant,
    #[msg("Raffle already drawn")]
    RaffleAlreadyDrawn,
    #[msg("Raffle not drawn")]
    RaffleNotDrawn,
    #[msg("Invalid winner index")]
    InvalidWinnerIndex,
    #[msg("Invalid claimant")]
    InvalidClaimant,
}
