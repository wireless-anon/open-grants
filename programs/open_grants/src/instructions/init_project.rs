use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(config: ProjectConfig)]
pub struct InitProject<'info> {
  /// === derived ===

  #[account(
    init,
    payer = signer,
    space = Project::LEN,
    // Could be a pda w/ seed equal to some global id, idk
  )]
  pub project: Account<'info, Project>,

  #[account(address = config.user)]
  pub user: Account<'info, User>,

  // You may not init a project account on behalf of others
  // to prevent malicious actors from spoofing projects
  #[account(
    mut,
    address = user.authority,
  )]
  pub signer: Signer<'info>,

  pub system_program: Program<'info, System>,
}

pub fn init_project_handler(ctx: Context<InitProject>, config: ProjectConfig) -> Result<()> {
  let project = &mut ctx.accounts.project;
  let clock = Clock::get()?;

  **project = Project {
    config,
    created_at: clock.unix_timestamp,
    updated_at: clock.unix_timestamp,
  };

  Ok(())
}
