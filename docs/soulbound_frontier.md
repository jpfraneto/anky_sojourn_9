# Soulbound Frontier

This note states what must become true in code for the constitutional soulbound model to become true in implementation.

## Current Truth

The constitution already assumes:

- one wallet
- one vessel
- one daily crossing

The current program does not yet make that fully true in code.

## What Must Be Added

### `claim_vessel` / `mint_vessel`

- claim through the canonical app flow while supply remains
- deliberate claim action
- exactly one vessel assigned to a wallet
- no second claim for the same wallet
- no second claim for the same vessel

### Supply Tracking

- onchain tracking for the 3,456-vessel cap
- claim closure once supply is exhausted

### Stewardship Enforcement

- a wallet stewardship record
- a vessel claim record
- one-wallet / one-vessel enforcement at claim time

### Seal Simplification

- remove unnecessary selected-vessel ambiguity from the seal path
- keep the seal path commitment-based
- continue to verify stewardship without exposing writing

## What Can Stay

- `initialize_season`
- UTC day derivation
- chamber derivation
- the `DailySeal` commitment model
- reward eligibility recording at seal time

## What Should Later Simplify

- `selected_vessel_*` naming
- the vessel-keyed V1 seal surface
- any repo language that treats vessel selection as a normal seasonal action

## Not This

This frontier is not about:

- marketplace expansion
- social-feed expansion
- protocol-layer AI expansion

It is about making stewardship, continuity, and private writing true in code.
