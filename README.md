# SOJOURN 9

SOJOURN 9 is a finite daily rite of return.

It is a 96-day writing pilgrimage divided into 12 regions of 8 days, entered through the iOS app, and bounded by exactly 3,456 vessels. Each vessel is a soulbound cNFT. One wallet may steward at most one vessel.

**The rite is private. The proof is public. The crossing is daily.**

This repo is the constitutional and public-proof seed of Sojourn 9. `constitution/SOJOURN_9.md` is the operative source of truth.

## Canonical Human Act

1. Claim vessel through the iOS app while supply remains.
2. Write one raw `.anky`.
3. Compute the hash on device.
4. Seal the commitment onchain.
5. Continue the crossing.

## Rite, Proof, Implementation

- `Rite`: the lived daily act of claim, writing, sealing, return, and continuity across 96 days.
- `Proof`: the public commitment layer that verifies stewardship and chamber closure without exposing the writing.
- `Implementation`: the current code embodiment in this repo.

## Non-goals

- a general journaling app
- an open social writing feed
- an NFT marketplace-first system
- a speculation-first system
- a protocol for publishing raw writing onchain
- a broad AI writing assistant at the protocol layer

## Repository

- `constitution/`: the law of the season
- `docs/`: the engineering interpretation of the law
- `program/`: the Anchor workspace and Solana program scaffold
- `clients/`: notes and placeholder surfaces, not a full client
- `deployments/`, `artifacts/`, `idl/`: placeholder and manifest layers

## Current Surface

- the implemented program surface is `initialize_season`, `seal_anky`, and scaffolded `claim_reward`
- `initialize_season` commits season constants and the constitutional hash
- `seal_anky` performs UTC day derivation, chamber derivation, Bubblegum collection-membership verification, current-steward verification, creation of vessel-day `DailySeal`, and emission of `AnkySealed`
- the chain records commitment, not content
- raw `.anky`, reflected `.anky`, and private archive remain offchain

## Current Limitation

- live V1 still uses `selected_vessel_asset_id`
- live V1 still uses a vessel-keyed PDA
- this remains transitional until `claim_vessel` exists

## Not Yet Implemented

- no implemented `claim_vessel`
- no implemented `mint_vessel`
- no onchain 3,456 supply tracking yet
- no one-wallet-per-vessel enforcement record yet
- `claim_reward` is still scaffold / TODO
- no reviewed committed IDL
- no published program IDs
- no immutable Arweave snapshot yet
- no actual iOS app code in this repo
- the TS SDK remains a placeholder / helper surface

## Minimum Path To First True Ship

- [ ] implement `claim_vessel`
- [ ] enforce the `3,456` hard cap onchain
- [ ] enforce one-wallet / one-vessel stewardship
- [ ] finalize the canonical seal path
- [ ] commit a reviewed public IDL
- [ ] publish program IDs
- [ ] prove the full devnet flow end-to-end
- [ ] align the iOS client to the canonical human act

## Local Development

```bash
npm install
npm run build
npm run test
npm run anchor:test
```

Useful commands:

```bash
npm run fmt
npm run lint
npm run idl:sync
```
