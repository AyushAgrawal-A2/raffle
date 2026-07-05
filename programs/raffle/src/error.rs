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
    #[msg("Randomness already requested")]
    RandomnessAlreadyRequested,
    #[msg("Invalid randomness account")]
    InvalidRandomnessAccount,
    #[msg("Wrong randomness account")]
    WrongRandomnessAccount,
    #[msg("Randomness expired")]
    RandomnessExpired,
    #[msg("Randomness not resolved")]
    RandomnessNotResolved,
    #[msg("Raffle not drawn")]
    RaffleNotDrawn,
    #[msg("Invalid winner index")]
    InvalidWinnerIndex,
    #[msg("Invalid claimant")]
    InvalidClaimant,
}
