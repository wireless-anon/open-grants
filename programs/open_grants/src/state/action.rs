use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;

use crate::ID;

#[account]
pub struct Action {
  pub instruction: ActionInstruction,
  pub approved: Vec<bool>,
  pub executed: bool,
}

impl Action {
  pub const LEN: usize = 1024;

  pub fn approval_count(&self) -> u8 {
    self.approved.iter().filter(|&&b| b).count() as u8
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ActionInstruction {
  // program id is always calling program
  pub accounts: Vec<ActionAccount>,
  pub data: Vec<u8>,
}

impl From<ActionInstruction> for Instruction {
  fn from(action: ActionInstruction) -> Self {
    Self {
      program_id: ID,
      accounts: action.accounts.into_iter().map(|a| a.into()).collect(),
      data: action.data,
    }
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq)]
pub struct ActionAccount {
  pub pubkey: Pubkey,
  pub is_signer: bool,
  pub is_writable: bool,
}

impl From<ActionAccount> for AccountMeta {
  fn from(account: ActionAccount) -> Self {
    Self {
      pubkey: account.pubkey,
      is_signer: account.is_signer,
      is_writable: account.is_writable,
    }
  }
}
