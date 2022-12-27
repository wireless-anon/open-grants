use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct UpdateProject<'info> {
  #[account(mut)]
  pub project: Account<'info, Project>,

  /// === derived ===

  #[account(address = project.config.user)]
  pub user: Account<'info, User>,

  #[account(address = user.authority)]
  pub signer: Signer<'info>,
}

pub fn update_project_handler(ctx: Context<UpdateProject>, config: ProjectConfig) -> Result<()> {
  let project = &mut ctx.accounts.project;
  let clock = Clock::get()?;

  **project = Project {
    config,
    created_at: project.created_at,
    updated_at: clock.unix_timestamp,
  };

  Ok(())
}
