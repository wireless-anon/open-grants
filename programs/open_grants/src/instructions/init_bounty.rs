use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(config: BountyConfig)]
pub struct InitBounty<'info> {
  /// === derived ===

  #[account(
    init,
    payer = signer,
    space = Bounty::LEN,
  )]
  pub bounty: Account<'info, Bounty>,

  #[account(address = config.user)]
  pub user: Account<'info, User>,

  // You may not init a bounty on behalf of others
  #[account(
    mut,
    address = user.authority,
  )]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn init_bounty_handler(ctx: Context<InitBounty>, config: BountyConfig) -> Result<()> {
  let bounty = &mut ctx.accounts.bounty;
  let clock = Clock::get()?;

  **bounty = Bounty {
    config,
    created_at: clock.unix_timestamp,
    updated_at: clock.unix_timestamp,
  };

  Ok(())
}
