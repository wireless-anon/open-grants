use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(authority: UserAuthority)]
pub struct InitUser<'info> {
  // You may init a user account on behalf of others
  #[account(mut)]
  pub signer: Signer<'info>,

  /// === derived ===

  #[account(
    init,
    payer = signer,
    space = User::LEN,
    seeds = [
      b"user",
      authority.key().as_ref(),
    ],
    bump,
  )]
  pub user: Account<'info, User>,

  pub system_program: Program<'info, System>,
}

pub fn init_user_handler(ctx: Context<InitUser>, authority: UserAuthority) -> Result<()> {
  let user = &mut ctx.accounts.user;

  **user = User { authority };

  Ok(())
}
