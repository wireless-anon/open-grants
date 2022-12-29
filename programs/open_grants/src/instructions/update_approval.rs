use anchor_lang::prelude::*;

use crate::errors::*;
use crate::state::*;

#[derive(Accounts)]
pub struct UpdateApproval<'info> {
  #[account(mut)]
  pub action: Account<'info, Action>,

  pub user: Account<'info, User>,

  #[account(constraint = user.is_signer(&signer.key()))]
  pub signer: Signer<'info>,
}

pub fn update_approval_handler(ctx: Context<UpdateApproval>, approval: bool) -> Result<()> {
  let action = &mut ctx.accounts.action;
  let user = &ctx.accounts.user;

  let UserAuthority::Multi(multisig) = &user.authority else {
    return Err(OpenGrantsError::Unknown.into());
  };

  let Some(index) = multisig.signers().iter().position(|signer| signer == &user.key()) else {
    return Err(OpenGrantsError::Unknown.into());
  };

  action.approved[index] = approval;

  Ok(())
}
