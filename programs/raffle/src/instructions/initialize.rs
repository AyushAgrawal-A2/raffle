use anchor_lang::prelude::*;

use crate::{constants::*, state::Counter};

#[derive(Accounts)]
pub struct Initialize<'info> {
    payer: Signer<'info>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    Ok(())
}
