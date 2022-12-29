use anchor_lang::prelude::*;

/**
Metadata JSON format:
{
  "name": "Bounty Name",
  "description": "Bounty Description",
  "image": "https://example.com/image.png",
  "website": "https://example.com",
  "category": "Dev Tools",
  "tags": [
    "tag1",
    "tag2",
    "...",
  ],
}
*/
#[account]
pub struct Bounty {
  pub config: BountyConfig,
  pub created_at: i64,
  pub updated_at: i64,
}

impl Bounty {
  pub const LEN: usize = 8 + 8 + 32 + 32 + 256 + 32 + 8 + 8;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub struct BountyConfig {
  pub active: bool, // frontend-only
  pub user: Pubkey,
  pub project: Pubkey, // can be 0, for bounties not associated with a project
  pub metadata_uri: String, // len: 256
  pub awardee: Pubkey,
}
