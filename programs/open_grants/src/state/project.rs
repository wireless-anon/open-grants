use anchor_lang::prelude::*;

/**
Metadata JSON format:
{
  "name": "Project Name",
  "description": "Project Description",
  "image": "https://example.com/image.png",
  "website": "https://example.com",
  "category": "Dev Tools",
  "repository": "https://github.com/example",
  "community": {
    "twitter": "https://twitter.com/example",
    "discord": "https://discord.com/invite/example",
    "tiktok": "https://tiktok.com/example",
  },
  "tags": [
    "tag1",
    "tag2",
    "...",
  ],
}
*/
#[account]
pub struct Project {
  pub config: ProjectConfig,
  pub created_at: i64,
  pub updated_at: i64,
}

impl Project {
  pub const LEN: usize = 8 + 32 + 256 + 32;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub struct ProjectConfig {
  pub user: Pubkey,
  pub metadata_uri: String, // len: 256
}
