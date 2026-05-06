# Sojourn 9 Decisions

This file records implementation-facing invariants derived from `constitution/SOJOURN_9.md`.

If any item here conflicts with the constitution, the constitution wins.

## D-001 Operative Source

- `constitution/SOJOURN_9.md` is the operative constitutional source text in this repo.

## D-002 Season Shape

- Sojourn 9 contains 96 chambers, 12 regions of 8 chambers each, and 3,456 canonical vessels.
- The canonical day is UTC.
- A vessel may seal at most once per UTC day, and only the current chamber may be sealed.

## D-003 Canonical Entry

- The iOS app is the canonical entrance.
- While supply remains, the app invites an unclaimed wallet to claim its vessel through a deliberate action.
- After claim, today’s chamber is the first recurring action.
- The season is designed as continuity, not episodic engagement.

## D-004 Stewardship Law

- Each valid vessel is a soulbound cNFT in the canonical Sojourn 9 collection.
- A wallet may steward at most one valid vessel in the season.
- Once claimed, a vessel remains bound to its claiming wallet for the duration of the season.
- The canonical surface should not introduce selected-vessel ambiguity.

## D-005 Seal And Privacy Law

- The writer’s device computes a hash from the exact raw `.anky` string.
- The program receives only that hash commitment, not plaintext `.anky`.
- The chain records commitment, not content.
- The rite is private and the proof is public.
- The chain exists to verify commitment and stewardship, not to expose writing.
- Vessel history and writer history remain distinct.
- Reward follows the valid seal, not mere possession.

## D-006 Current Program Surface

- The current implemented surface is `initialize_season`, `seal_anky`, and scaffolded `claim_reward`.
- `initialize_season` commits season constants and the constitutional hash.
- `seal_anky` performs UTC day derivation, chamber derivation, Bubblegum collection-membership verification, current-steward verification, creation of vessel-day `DailySeal`, and emission of `AnkySealed`.
- Live V1 still uses `selected_vessel_asset_id` and a vessel-keyed PDA.
- That V1 shape remains transitional until `claim_vessel` exists.

## D-007 Repo Framing

- Repo-facing documents must keep the distinction between rite, proof, and implementation clear.
- Repo-facing documents must not frame Sojourn 9 as a general journaling app, an open social writing feed, an NFT marketplace-first surface, a speculation-first system, a protocol for publishing raw writing onchain, or a broad AI writing assistant at the protocol layer.
