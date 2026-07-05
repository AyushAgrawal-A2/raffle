use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{constants::*, error::RaffleError, state::Raffle};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Create<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = creator,
        associated_token::token_program = token_program,
    )]
    creator_token_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = creator,
        space = Raffle::space(0),
        seeds = [RAFFLE_SEED, creator.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    raffle: Account<'info, Raffle>,

    #[account(
        init,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = raffle,
        associated_token::token_program = token_program,
    )]
    raffle_token_ata: InterfaceAccount<'info, TokenAccount>,

    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

pub fn handle_create(
    ctx: Context<Create>,
    id: u64,
    start: i64,
    end: i64,
    amount: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    require!(start > current_time, RaffleError::InvalidArguments);
    require!(end > start, RaffleError::InvalidArguments);
    require!(amount > 0, RaffleError::InvalidArguments);
    ctx.accounts.raffle.set_inner(Raffle {
        id,
        creator: ctx.accounts.creator.key(),
        start,
        end,
        entrants: Vec::new(),
        winner: None,
        status: crate::RaffleStatus::Open,
        randomness_account: Pubkey::default(),
        bump: ctx.bumps.raffle,
    });
    token_interface::transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token_interface::TransferChecked {
                from: ctx.accounts.creator_token_ata.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.raffle_token_ata.to_account_info(),
                authority: ctx.accounts.creator.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;
    Ok(())
}
