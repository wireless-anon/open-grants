use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateBounty<'info> {
  #[account(mut)]
  pub bounty: Account<'info, Bounty>,

  /// === derived ===

  #[account(address = bounty.config.user)]
  pub user: Account<'info, User>,

  #[account(address = user.authority.key())]
  pub signer: Signer<'info>,
}

pub fn update_bounty_handler(ctx: Context<UpdateBounty>, config: BountyConfig) -> Result<()> {
  let bounty = &mut ctx.accounts.bounty;
  let clock = Clock::get()?;

  **bounty = Bounty {
    config,
    created_at: bounty.created_at,
    updated_at: clock.unix_timestamp,
  };

  Ok(())
}
