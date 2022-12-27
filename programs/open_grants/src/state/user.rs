use anchor_lang::prelude::*;

// Could add a metadata_uri for profile information, or just
// pull from canonical identity service (e.g. Backpack) in
// the frontend
#[account]
pub struct User {
  pub authority: Pubkey,
}

impl User {
  pub const LEN: usize = 8 + 32;
}
