use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitUser<'info> {
  // You may init a user account on behalf of others
  // Included here rather than ix params to take
  // advantage of deduplication with other ixs
  #[doc = "CHECK: Any account"]
  pub authority: UncheckedAccount<'info>,

  #[account(mut)]
  pub signer: Signer<'info>,

  /// === derived ===

  #[account(
    init,
    payer = signer,
    space = User::LEN,
    seeds = [
      b"user".as_ref(),
      authority.key().as_ref()
    ],
    bump,
  )]
  pub user: Account<'info, User>,

  pub system_program: Program<'info, System>,
}

pub fn init_user_handler(ctx: Context<InitUser>) -> Result<()> {
  let user = &mut ctx.accounts.user;
  let authority = &ctx.accounts.authority;

  **user = User {
    authority: authority.key(),
  };

  Ok(())
}
