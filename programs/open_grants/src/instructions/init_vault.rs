use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::ID;

// Maybe should have a specific instruction to init a vault for each type of account?
#[derive(Accounts)]
pub struct InitVault<'info> {
  pub mint: Account<'info, Mint>,

  #[account(owner = ID)]
  #[doc = "CHECK: Any account owned by the program (e.g. project, bounty, etc.)"]
  pub owner: UncheckedAccount<'info>,

  #[account(mut)]
  pub signer: Signer<'info>,

  /// === derived ===

  #[account(
    init,
    payer = signer,
    seeds = [
      b"vault",
      owner.key().as_ref(),
      mint.key().as_ref()
    ],
    bump,
    token::mint = mint,
    token::authority = owner,
  )]
  pub vault: Account<'info, TokenAccount>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

pub fn init_vault_handler(_ctx: Context<InitVault>) -> Result<()> {
  Ok(())
}
