use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawGrant<'info> {
  pub project: Account<'info, Project>,

  #[account(mut)]
  pub destination: Account<'info, TokenAccount>,

  /// === derived ===

  #[account(
    mut,
    seeds = [
      b"vault",
      project.key().as_ref(),
      mint.key().as_ref()
    ],
    bump,
    token::mint = mint,
    token::authority = project,
  )]
  pub vault: Account<'info, TokenAccount>,

  #[account(address = destination.mint)]
  pub mint: Account<'info, Mint>,

  #[account(address = project.config.user)]
  pub user: Account<'info, User>,

  #[account(address = user.authority.key())]
  pub signer: Signer<'info>,

  pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawGrant<'info> {
  pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.vault.to_account_info(),
      to: self.destination.to_account_info(),
      authority: self.project.to_account_info(),
    };

    let cpi_program = self.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    cpi_ctx
  }
}

pub fn withdraw_grant_handler(ctx: Context<WithdrawGrant>, amount: u64) -> Result<()> {
  let project = &ctx.accounts.project;
  let mint = &ctx.accounts.mint;

  let project_key = project.key();
  let mint_key = mint.key();
  let seeds = &[b"vault", project_key.as_ref(), mint_key.as_ref()];

  token::transfer(ctx.accounts.transfer_ctx().with_signer(&[seeds]), amount)?;

  Ok(())
}
