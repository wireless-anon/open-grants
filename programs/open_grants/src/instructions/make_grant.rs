use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::state::*;

#[derive(Accounts)]
pub struct MakeGrant<'info> {
  pub project: Account<'info, Project>,

  pub user: Account<'info, User>,

  #[account(
    mut,
    token::authority = signer,
  )]
  pub source: Account<'info, TokenAccount>,

  /// === derived ===

  #[account(
    init,
    payer = signer,
    space = Grant::LEN,
    // Could be pda with e.g. incrementing index from project as seed, idk
  )]
  pub grant: Account<'info, Grant>,

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

  #[account(address = source.mint)]
  pub mint: Account<'info, Mint>,

  #[account(
    mut,
    // Could theoretically be any account, but might as well
    // restrict to user since the whole point of the user account
    // is for on-chain record keeping
    address = user.authority,
  )]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>,

  pub token_program: Program<'info, Token>,
}

impl<'info> MakeGrant<'info> {
  pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
    let cpi_accounts = Transfer {
      from: self.source.to_account_info(),
      to: self.vault.to_account_info(),
      authority: self.project.to_account_info(),
    };

    let cpi_program = self.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    cpi_ctx
  }
}

pub fn make_grant_handler(
  ctx: Context<MakeGrant>,
  amount: u64,
  metadata_uri: String,
) -> Result<()> {
  let grant = &mut ctx.accounts.grant;
  let project = &ctx.accounts.project;
  let user = &ctx.accounts.user;
  let mint = &ctx.accounts.mint;
  let clock = Clock::get()?;

  **grant = Grant {
    project: project.key(),
    user: user.key(),
    amount,
    mint: mint.key(),
    metadata_uri,
    created_at: clock.unix_timestamp,
  };

  token::transfer(ctx.accounts.transfer_ctx(), amount)?;

  Ok(())
}
