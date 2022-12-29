use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct ExecuteAction<'info> {
  #[account(
    mut,
    constraint = !action.executed,
  )]
  pub action: Account<'info, Action>,

  pub user: Account<'info, User>,

  // Must be executed by one of the signers
  #[account(constraint = user.is_signer(&signer.key()))]
  pub signer: Signer<'info>,

  /// === derived ===

  #[doc = "CHECK: multisig authority"]
  pub authority: UncheckedAccount<'info>,
}

pub fn execute_action_handler(ctx: Context<ExecuteAction>) -> Result<()> {
  let action = &mut ctx.accounts.action;
  let user = &ctx.accounts.user;

  let UserAuthority::Multi(multisig) = &user.authority else {
    return Err(OpenGrantsError::Unknown.into());
  };

  if action.approval_count() < multisig.threshold {
    return Err(OpenGrantsError::Unknown.into());
  }

  action.executed = true;

  invoke_signed(
    &action.instruction.clone().into(),
    ctx.remaining_accounts,
    &[multisig_seeds!(multisig)],
  )?;

  Ok(())
}
