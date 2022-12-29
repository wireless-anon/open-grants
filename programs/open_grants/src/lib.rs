use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod errors;
mod instructions;
mod state;

pub use errors::*;
use instructions::*;
pub use state::*;

// Completely untested, but this is the general idea.

#[program]
pub mod open_grants {
  use super::*;

  pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    init_user_handler(ctx)
  }

  pub fn init_project(ctx: Context<InitProject>, config: ProjectConfig) -> Result<()> {
    init_project_handler(ctx, config)
  }

  pub fn update_project(ctx: Context<UpdateProject>, config: ProjectConfig) -> Result<()> {
    update_project_handler(ctx, config)
  }

  pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
    init_vault_handler(ctx)
  }

  pub fn make_grant(ctx: Context<MakeGrant>, amount: u64, metadata_uri: String) -> Result<()> {
    make_grant_handler(ctx, amount, metadata_uri)
  }

  pub fn withdraw_grant(ctx: Context<WithdrawGrant>, amount: u64) -> Result<()> {
    withdraw_grant_handler(ctx, amount)
  }

  pub fn init_bounty(ctx: Context<InitBounty>, config: BountyConfig) -> Result<()> {
    init_bounty_handler(ctx, config)
  }

  pub fn update_bounty(ctx: Context<UpdateBounty>, config: BountyConfig) -> Result<()> {
    update_bounty_handler(ctx, config)
  }

  pub fn withdraw_award(ctx: Context<WithdrawAward>, amount: u64) -> Result<()> {
    withdraw_award_handler(ctx, amount)
  }
}
