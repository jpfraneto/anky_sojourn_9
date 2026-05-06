# Sojourn 9 TypeScript SDK

This package is a placeholder support library for the Sojourn 9 public-proof surface.

It is not the rite.
It is not a full client.

Current scope:

- constitutional constants such as chamber count, region count, vessel count, and UTC day seconds
- UTC day, chamber index, and region index helpers
- TypeScript interfaces that mirror the scaffolded public-proof accounts
- placeholder interfaces for the current V1 `seal_anky` proof boundary and argument shape
- terminology aligned to valid vessel, claimed vessel, current steward, and device-computed `.anky` hash commitments

Current limitation:

- some exported V1 fields still use `selectedVessel*` names to match the current onchain instruction surface
- those names are transitional and should not be read as season law

Future scope:

- PDA derivation helpers once account layouts are stable
- typed instruction builders for Bubblegum proof submission once the SDK grows beyond placeholders
- claim-vessel and stewardship helpers once the soulbound claim surface exists
- types generated from reviewed public IDL snapshots
