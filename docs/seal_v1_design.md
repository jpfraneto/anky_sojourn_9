# Seal V1 Design Note

This note describes the current `seal_anky` shape.

It is a transitional implementation note.
It is not the constitution.

## Purpose

`seal_anky` records the public commitment that a valid vessel closed the current chamber.

It does not publish the writing.
The chain records commitment, not content.

## Current Inputs

- `selected_vessel_asset_id`
- `anky_hash_commitment`
- signer: `current_steward`
- Bubblegum proof material for the vessel

The `selected_vessel_*` names are transitional.

## Current Checks

V1 currently checks:

- UTC day from onchain time
- chamber index from season start
- Bubblegum collection-membership proof against `SeasonConfig.vessel_collection`
- proved vessel owner equals `current_steward`
- no prior daily seal for that vessel on that UTC day

## Current Output

V1 currently:

- creates a `DailySeal`
- records reward eligibility
- emits `AnkySealed`

The daily seal PDA is currently derived from:

- `season_config`
- `selected_vessel_asset_id`
- `utc_day`

This is the live vessel-keyed V1 shape.

## What V1 Does Not Yet Do

- implement `claim_vessel`
- implement `mint_vessel`
- enforce the 3,456 hard cap onchain
- enforce one-wallet / one-vessel stewardship onchain
- derive the claimed vessel from claim state instead of receiving `selected_vessel_*` inputs

## Boundary

This instruction is proof machinery around the rite.
It is not the rite itself.
