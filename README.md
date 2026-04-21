# SOJOURN 9

SOJOURN 9 is the ninth canonical season of Anky: a 96-chamber writing pilgrimage crossed through the iOS app, one UTC day at a time, by 3,456 canonical vessels on Solana.

`constitution/SOJOURN_9.md` is the operative source of truth in this repository. The code and metadata here exist to preserve the public proof layer that the constitution requires. They are not the rite itself, and they do not outrank the constitutional text.

## Constitutional Baseline

- the iOS app is the canonical entrance
- today's chamber is the canonical first action
- the season contains 96 chambers arranged as 12 regions of 8 chambers each
- each vessel may seal at most one Anky per UTC day
- the seal right lives in the vessel and follows current stewardship
- private `.anky` writing and reflected `.anky` stay offchain and do not transfer with the vessel
- reward follows the valid seal, not later vessel possession

## Canonical Vs. Implementation-Specific

Canonical material in this repository includes the constitutional text, the decision log derived from it, the constitutional changelog, deployment manifests that point back to the active constitutional source, and artifact records for immutable uploads once they exist.

Implementation-specific material includes the Anchor workspace, Rust modules, SDK helpers, and developer tooling. These should preserve constitutional law without confusing implementation detail for the canonical surface.

## Repository Layout

```text
constitution/   Constitutional text, derivative decision records, changelog, and snapshot notes
program/        Anchor workspace and the Sojourn 9 public proof program
idl/            Reviewed public IDL snapshots and release notes
clients/        Support SDK and app-surface notes for non-constitutional implementation work
deployments/    Devnet and mainnet deployment manifests keyed to the active constitution
artifacts/      Artifact references such as Arweave mappings for immutable constitutional copies
docs/           Public-proof design notes and implementation planning
scripts/        Small developer utilities
```

## Local Development

Tooling currently installed on this machine:

- `anchor-cli 0.31.1`
- `rustc 1.93.0`
- `cargo 1.93.0`
- `solana-cli 3.1.12`
- `node v22.22.0`
- `npm 10.9.4`

Quickstart:

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

## Current Implementation Surface

The constitutional text is complete enough to govern the scaffold, but the protocol implementation is still early.

Present in the repository:

- the operative constitutional text plus aligned decision and changelog records
- an Anchor workspace with season, vessel-day seal, and reward-claim account scaffolding
- deployment and artifact manifests that point to the active constitutional source by path and hash
- a TypeScript support package with constitutional constants and UTC/chamber helpers

Still open:

- cNFT ownership verification and transfer-aware seal enforcement
- canonical vessel collection or registry proof logic
- reward pool accounting, rollover treatment, and anti-replay claim enforcement
- reviewed public IDL snapshots committed under `idl/`
- deployed program IDs and immutable Arweave uploads

## Onchain Data Boundary

The Solana program is intended to store only minimum public truth: vessel identity, season identity, chamber/day identity, seal timing, steward-at-seal, and reward eligibility. The written `.anky`, the reflected `.anky`, and the writer's private archive remain offchain.
