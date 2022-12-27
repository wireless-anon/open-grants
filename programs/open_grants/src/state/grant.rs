use anchor_lang::prelude::*;

/**
Metadata JSON format:
{
  "name": "Grant Name",
  "description": "Grant Description",
  "tags": ["tag1", "tag2", ...],
}
*/
#[account]
pub struct Grant {
  pub project: Pubkey,
  pub user: Pubkey,
  pub amount: u64,
  pub mint: Pubkey,
  pub metadata_uri: String, // len: 256
  pub created_at: i64,
}

impl Grant {
  pub const LEN: usize = 8 + 32 + 8 + 32 + 256 + 8;
}
