use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawAward<'info> {
  pub bounty: Account<'info, Bounty>,

  #[account(mut)]
  pub destination: Account<'info, TokenAccount>,

  // Either the user or the awardee can withdraw from a bounty.
  #[account(
    constraint = (
      user.key() == bounty.config.user ||
      user.key() == bounty.config.awardee
    )
  )]
  pub user: Account<'info, User>,

  /// === derived ===

  #[account(
    mut,
    seeds = [
      b"vault",
      bounty.key().as_ref(),
      mint.key().as_ref()
    ],
    bump,
    token::mint = mint,
    token::authority = bounty,
  )]
  pub vault: Account<'info, TokenAccount>,

  #[account(address = destination.mint)]
  pub mint: Account<'info, Mint>,

  #[account(address = user.authority)]
  pub signer: Signer<'info>,

  pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawAward<'info> {
  pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.vault.to_account_info(),
      to: self.destination.to_account_info(),
      authority: self.bounty.to_account_info(),
    };

    let cpi_program = self.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    cpi_ctx
  }
}

pub fn withdraw_award_handler(ctx: Context<WithdrawAward>, amount: u64) -> Result<()> {
  let bounty = &ctx.accounts.bounty;
  let mint = &ctx.accounts.mint;

  let bounty_key = bounty.key();
  let mint_key = mint.key();
  let seeds = &[b"vault", bounty_key.as_ref(), mint_key.as_ref()];

  token::transfer(ctx.accounts.transfer_ctx().with_signer(&[seeds]), amount)?;

  Ok(())
}
