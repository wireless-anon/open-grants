use anchor_lang::prelude::*;

#[error_code]
pub enum GrantsError {
  #[msg("Unknown")]
  Unknown,
}
