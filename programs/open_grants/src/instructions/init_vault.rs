use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::state::*;

#[derive(Accounts)]
pub struct InitVault<'info> {
  pub mint: Account<'info, Mint>,

  pub project: Account<'info, Project>,

  #[account(mut)]
  pub signer: Signer<'info>,

  /// === derived ===

  #[account(
    init,
    payer = signer,
    seeds = [
      b"vault".as_ref(),
      project.key().as_ref(),
      mint.key().as_ref()
    ],
    bump,
    token::mint = mint,
    token::authority = project,
  )]
  pub vault: Account<'info, TokenAccount>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

pub fn init_vault_handler(_ctx: Context<InitVault>) -> Result<()> {
  Ok(())
}
