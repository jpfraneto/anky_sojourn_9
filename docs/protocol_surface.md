# Sojourn 9 Public Proof Surface

Sojourn 9 is a finite daily rite of return.
This document concerns the proof layer only.

The rite is private.
The proof is public.

The Solana program exists to verify stewardship and chamber closure without exposing the writing itself.

## Hard Constraints

- 96 chambers, 12 regions of 8 chambers each
- exactly 3,456 vessel cNFTs
- canonical entrance through the iOS app
- claim through the app while supply remains
- one wallet may steward at most one valid vessel
- one seal per vessel per UTC day
- UTC day law
- no retroactive sealing
- the chain records commitment, not content

## Current Proof Accounts

### `SeasonConfig`

Current fields:

- season number
- season start timestamp
- chamber count
- region count
- chambers per region
- canonical vessel count
- vessel collection
- constitutional revision
- constitutional hash

Current limitation:

- no claimed-supply tracking yet
- no wallet stewardship record yet

### `DailySeal`

Current fields:

- season config reference
- vessel asset id
- UTC day
- chamber index
- current steward
- `.anky` hash commitment
- seal timestamp
- reward eligibility flag

Current limitation:

- the live field name is still `selected_vessel_asset_id`

### `RewardClaim`

Current status:

- scaffold only

## Current Proof Instructions

### `initialize_season`

Current role:

- commit season constants
- commit the constitutional hash

### `seal_anky`

Current role:

- derive the UTC day
- derive the chamber index
- verify Bubblegum collection membership
- verify current stewardship
- create the vessel-day `DailySeal`
- emit `AnkySealed`

Current V1 shape:

- input still includes `selected_vessel_asset_id`
- the daily seal PDA is still vessel-keyed

This remains transitional until `claim_vessel` exists.

### `claim_reward`

Current status:

- scaffold only
- still returns `UnimplementedInstruction`

## What Does Not Yet Exist

- no implemented `claim_vessel`
- no implemented `mint_vessel`
- no onchain 3,456 supply tracking yet
- no one-wallet-per-vessel enforcement record yet
- no reviewed committed IDL
- no published program IDs
- no immutable Arweave snapshot
- no actual iOS app code in this repo

## Frontier

The next frontier is simple:

- implement `claim_vessel`
- enforce the 3,456 hard cap
- enforce one-wallet / one-vessel stewardship
- keep the seal path commitment-based
- remove unnecessary `selected_vessel_*` ambiguity from the public surface

## Non-goals Of The Proof Layer

- storing raw `.anky` or reflected `.anky` onchain
- acting as a social publishing layer
- acting as a marketplace-first or speculation-first layer
- acting as a broad AI writing assistant at the protocol layer
