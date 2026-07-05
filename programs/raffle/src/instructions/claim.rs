use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{constants::*, error::RaffleError, state::Raffle};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Claim<'info> {
    #[account(mut)]
    winner: Signer<'info>,

    #[account(mut)]
    /// CHECK: checked via pda seeds and has_one constraint
    creator: UncheckedAccount<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = winner,
        associated_token::mint = mint,
        associated_token::authority = winner,
        associated_token::token_program = token_program,
    )]
    winner_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = creator,
        has_one = creator,
        seeds = [RAFFLE_SEED, creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump = raffle.bump
    )]
    raffle: Account<'info, Raffle>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = raffle,
        associated_token::token_program = token_program,
    )]
    raffle_token_ata: InterfaceAccount<'info, TokenAccount>,

    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

pub fn handle_claim(ctx: Context<Claim>, id: u64) -> Result<()> {
    let winner_index = ctx
        .accounts
        .raffle
        .winner
        .ok_or(RaffleError::RaffleNotDrawn)?;
    let winner = ctx
        .accounts
        .raffle
        .entrants
        .get(winner_index as usize)
        .ok_or(RaffleError::InvalidWinnerIndex)?;
    require_keys_eq!(
        *winner,
        *ctx.accounts.winner.key,
        RaffleError::InvalidClaimant
    );

    let creator_address = ctx.accounts.creator.key();
    let seeds = [
        RAFFLE_SEED,
        creator_address.as_ref(),
        &id.to_le_bytes(),
        &[ctx.accounts.raffle.bump],
    ];
    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token_interface::TransferChecked {
                from: ctx.accounts.raffle_token_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.winner_token_ata.to_account_info(),
                authority: ctx.accounts.raffle.to_account_info(),
            },
            &[&seeds[..]],
        ),
        ctx.accounts.raffle_token_ata.amount,
        ctx.accounts.mint.decimals,
    )?;
    token_interface::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token_interface::CloseAccount {
            account: ctx.accounts.raffle_token_ata.to_account_info(),
            destination: ctx.accounts.creator.to_account_info(),
            authority: ctx.accounts.raffle.to_account_info(),
        },
        &[&seeds[..]],
    ))?;
    Ok(())
}
