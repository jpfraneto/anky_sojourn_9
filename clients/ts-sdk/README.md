# Sojourn 9 TypeScript SDK

This package is a support library for integrations that need the Sojourn 9 public-proof surface.

The canonical entrance remains the iOS app. This SDK exists to support developer workflows and secondary integrations without pretending to define the rite.

Current scope:

- constitutional constants such as chamber count, region count, vessel count, and UTC day seconds
- UTC day, chamber index, and region index helpers
- TypeScript interfaces that mirror the scaffolded public-proof accounts

Future scope:

- PDA derivation helpers once account layouts are stable
- types generated from reviewed public IDL snapshots
