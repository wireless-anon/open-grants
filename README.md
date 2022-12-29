# open-grants (work in progress)

An open protocol for giving grants on Solana.

## Summary

The goal of this project is to create an open protocol for giving grants on Solana. The protocol will be implemented as a Solana program, and will be accompanied by an SDK and an reference open-source frontend implementation.

It will be a public good with zero fees and it will be governed by a multisig of Solana ecosystem participants with the intention of eventually becoming immutable.

## Todo

- Discuss desired features
- Finalize specification v0
- Complete contract implementation
- Create Typescript SDK
- Create open-source frontend

## Specification v0

#### Partially Implemented

- **User**: A user is a participant in the protocol. It should be noted that, as the authority of a user account is an arbitrary pubkey, _user accounts can be multisigs or DAOs_.
- **Grant**: A payment made by a user to a project, with optional metadata. Grants can be made in any token mint, though frontends may choose to filter for only certain mints (e.g. USDC and wSOL) to avoid visual pollution from spam tokens. Grant metadata can include:
  - Name (e.g. "Solana Foundation Grant")
  - Description (e.g. "Grant to support Anchor development")
  - Tags
- **Project**: Projects are created by users to receive grants. Projects can be created arbitrarily, though frontends may employ a filtering system to ensure that only high-quality projects are shown (_see verification idea below_). Project metadata can include:
  - Name (e.g. "Anchor")
  - Description (e.g. "Anchor is a protocol for building on-chain programs.")
  - Image (e.g. "https://example.com/anchor-logo.png")
  - Website (e.g. "https://www.anchor-lang.com/");
  - Category (e.g. "Dev Tools")
  - Repository (e.g. "https://github.com/coral-xyz/anchor")
  - Community
    - Twitter
    - Discord
    - Telegram
    - TikTok
  - Tags
- **Bounty**: Bounties are created by users for other users to complete in order to receive some reward. Bounties may optionally be associated with a project. Like projects, a filtering system may be used by frontends to ensure that only high-quality bounties are shown. Bounty metadata can include:
  - Name (e.g. "Anchor IDL Realloc Support")
  - Description (e.g. "Looking for somebody to implement `realloc` for @anchorlang idl accounts.")
  - Image
  - Website (e.g. "https://twitter.com/crispheaney/status/1606027358955130880")
  - Category (e.g. "Dev Tools")
  - Tags
- **Metadata**: Metadata objects for grants, projects, bounties, etc. can be uploaded to Arweave using Bundlr Network. The storage payment can be made in SOL.
  - Example: https://github.com/jacobcreech/Token-Creator/blob/master/src/components/UploadMetadata.tsx#L124

#### Ideas

- **Verification**: The program upgrade authority may 'verify' a project. This would have no effect other than to serve as an optional signal for frontends to filter for high-quality projects. It is expected that the program upgrade authority will be gated by a multisig (e.g. using squads.so) of ecosystem participants.
  - Alternatively, could have multiple different 'verifiers' and frontends can choose to filter for projects verified by a certain verifier.
- **Voting**: Similar to Product Hunt, users can vote on projects. This would be a way to surface high-quality projects to the top of the list.
  - Susceptible to spam, could be mitigated by requiring some 'proof of work' to become a voter (e.g. only give voting powers to solana devs active on github, active community managers, people that go by their real identity, etc.)
- **User profiles**: Users can create a profile with a username, bio, image, etc.
  - Prefer to pull from identity standard (e.g. Backpack) over reinventing the wheel
- **Milestone grants**: Make a grant that can only be withdrawn after a milestone is reached (e.g. successful completion of a dev tool or game).
  - Need to figure out how to verify milestones.
  - Could assign a 'milestone oracle' to each milestone, and the milestone oracle can release the funds.

#### Post-MVP / Non-goals

- **Streaming grants**: Make a grant that is released over time (e.g. 1% per day).
  - In my opinion, not necessary for v0.
  - Most of these cases are covered by milestone grants or are more appropriate for a normal payment system (e.g. Streamflow). I think that grant systems should be based around payments for concrete deliverables.
