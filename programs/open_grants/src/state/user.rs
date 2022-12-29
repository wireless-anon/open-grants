use anchor_lang::prelude::*;

use crate::ID;

macro_rules! multisig_seeds {
  ($multisig:expr) => {
    &[
      &[b"user-authority" as &[u8], &[$multisig.threshold]],
      $multisig
        .signers()
        .iter()
        .map(Pubkey::as_ref)
        .collect::<Vec<_>>()
        .as_slice(),
    ]
    .concat()
  };
}

pub(crate) use multisig_seeds;

// Could add a metadata_uri for profile information, or just
// pull from canonical identity service (e.g. Backpack) in
// the frontend
#[account]
pub struct User {
  pub authority: UserAuthority,
  // TODO: add other stuff like reputation, total bounty amount received, etc.
}

impl User {
  pub const LEN: usize = 8 + 32;

  pub fn is_signer(&self, signer: &Pubkey) -> bool {
    match &self.authority {
      UserAuthority::Single(pk) => pk == signer,
      UserAuthority::Multi(Multisig { signers, .. }) => signers.contains(signer),
    }
  }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq)]
pub enum UserAuthority {
  Single(Pubkey),
  Multi(Multisig),
}

impl UserAuthority {
  pub fn key(&self) -> Pubkey {
    match self {
      UserAuthority::Single(pk) => *pk,
      UserAuthority::Multi(multisig) => {
        Pubkey::find_program_address(multisig_seeds!(multisig), &ID).0
      }
    }
  }
}

#[derive(AnchorSerialize, Clone, Debug, PartialEq)]
pub struct Multisig {
  // private field to ensure signers are sorted
  signers: Vec<Pubkey>,
  pub threshold: u8,
}

impl Multisig {
  pub fn new(signers: Vec<Pubkey>, threshold: u8) -> Self {
    let mut signers = signers;
    signers.sort();

    Self { signers, threshold }
  }

  pub fn signers(&self) -> &[Pubkey] {
    &self.signers
  }
}

impl AnchorDeserialize for Multisig {
  fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
    let signers = Vec::<Pubkey>::deserialize(buf)?;
    let threshold = u8::deserialize(buf)?;

    Ok(Self::new(signers, threshold))
  }
}
