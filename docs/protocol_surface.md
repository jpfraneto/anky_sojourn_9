# Sojourn 9 Public Proof Surface

This document describes the intended first onchain implementation pass that can coexist with `constitution/SOJOURN_9.md`.

The Solana program is not the map, the chamber, the return, or the iOS entrance. Its job is narrower: preserve the minimum public truth required for vessel stewardship, chamber closure, and reward entitlement without exposing private writing.

## Constitutional Constraints On The Program

- Sojourn 9 contains 96 chambers arranged as 12 regions of 8 chambers each.
- The iOS app remains the canonical entrance, so the program should stay focused on public proof rather than UI behavior.
- A vessel may seal at most one Anky per UTC day.
- Only the current chamber may be sealed. No retroactive and no future sealing is valid.
- Transfer preserves an unused daily right but does not create a second right after a valid seal.
- Private `.anky` writing and reflected `.anky` stay offchain.
- Reward follows the valid seal and stays with the steward who earned it at seal time.

## Planned Accounts

### `SeasonConfig`

Purpose:

- commit the constitutional season shape that the program recognizes
- bind the season to a canonical vessel collection or registry commitment
- commit a constitutional revision number and hash for auditability

Expected fields:

- Sojourn number
- season start timestamp aligned to a UTC day boundary
- chamber count, region count, and chambers-per-region constants
- canonical vessel count
- vessel collection or registry reference
- constitutional revision and constitutional hash

### `DailySeal`

Purpose:

- record that a specific vessel closed the current chamber on a specific UTC day

Expected fields:

- season config reference
- vessel asset id
- UTC day
- chamber index inside the 96-chamber path
- steward at seal time
- seal timestamp
- reward eligibility flag

### `RewardClaim`

Purpose:

- record that the reward attached to a valid seal has already been settled

Expected fields:

- referenced daily seal
- claimant or steward receiving settlement
- claim timestamp

## Planned Instructions

### `initialize_season`

Purpose:

- create the season configuration account
- bind the deployed proof surface to the active constitutional source

Likely checks:

- the supplied season number is `9`
- the supplied season start lands on a UTC day boundary
- the canonical vessel count is `3,456`
- the committed constitutional revision and hash match the deployment review process

Notes:

- authority should stay narrow
- constitutional shape data should be immutable once initialized

### `seal_anky`

Purpose:

- record the canonical closure of today's chamber for one vessel

Likely checks:

- the current timestamp resolves to a UTC day inside the 96-chamber season window
- the current UTC day maps to the correct chamber index for the season
- the vessel belongs to the Sojourn 9 vessel set
- the signer is the current steward of that vessel at seal time
- no prior seal exists for that vessel on that UTC day

Transfer semantics:

- if the vessel has not yet been used on that UTC day, transfer should preserve the right
- if a valid seal already exists for that vessel-day, transfer must not create another

Output:

- a single vessel-day seal record tied to the current chamber
- an event that downstream indexers can consume
- reward eligibility metadata for later settlement

### `claim_reward`

Purpose:

- claim reward for a previously valid seal

Likely checks:

- the referenced seal is valid and reward-eligible
- the claim path pays the steward who earned the reward at seal time
- the receipt account prevents duplicate settlement

Open design work:

- what asset or accounting system settles rewards
- whether empty-day reward allocation rolls forward directly or through an accumulator account
- whether claims are immediate, batched, delegated, or offchain-authorized

## PDA Notes

Likely derivations:

- season config: `["season", season_number_le_bytes]`
- daily seal: `["daily_seal", season, vessel_asset_id, utc_day_le_bytes]`
- reward claim receipt: `["reward_claim", daily_seal]`

These should remain stable once the first reviewed public IDL is published.

## What Is Still Open

- exact cNFT proof and ownership verification path
- collection versus explicit vessel registry design
- final reward pool and rollover mechanics
- whether any content commitment hash belongs onchain
- devnet and mainnet deployment identities
