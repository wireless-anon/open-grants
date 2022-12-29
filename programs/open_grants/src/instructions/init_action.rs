use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(instruction: ActionInstruction)]
pub struct InitAction<'info> {
  pub user: Account<'info, User>,

  #[account(
    mut,
    constraint = user.is_signer(&signer.key()),
  )]
  pub signer: Signer<'info>,

  /// === derived ===

  #[account(
    init,
    payer = signer,
    space = Action::LEN,
  )]
  pub action: Account<'info, Action>,

  pub system_program: Program<'info, System>,
}

pub fn init_action_handler(ctx: Context<InitAction>, instruction: ActionInstruction) -> Result<()> {
  let action = &mut ctx.accounts.action;

  **action = Action {
    instruction,
    approved: Default::default(),
    executed: false,
  };

  Ok(())
}
