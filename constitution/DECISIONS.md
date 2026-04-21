# Sojourn 9 Decision Log

This file records implementation-facing decisions derived from `constitution/SOJOURN_9.md`.

If any entry here conflicts with the constitution, the constitution wins.

## D-000: Operative Constitutional Source

- Status: `accepted`
- Date: `2026-04-21`
- Decision: `constitution/SOJOURN_9.md` is the operative constitutional source text for this repository until immutable snapshots are cut.
- Rationale: The repo scaffold, manifests, and code comments should derive from the constitutional text itself rather than from the earlier scaffold prompt.

## D-001: Season Shape Is Constitutional

- Status: `accepted`
- Date: `2026-04-21`
- Decision: Sojourn 9 contains exactly 96 chambers, 12 regions of 8 chambers each, and 3,456 canonical vessels.
- Rationale: These are season-defining constants, not tuning knobs for clients or deployments.

## D-002: The Canonical Entrance Is The iOS Chamber Flow

- Status: `accepted`
- Date: `2026-04-21`
- Decision: The iOS app is the canonical entrance and today's chamber is the canonical first action.
- Rationale: The repo may support public proof and secondary integrations, but it should not speak as if the season were centered on generic clients, feeds, or auxiliary surfaces.

## D-003: One Vessel-Based Seal Right Per UTC Day

- Status: `accepted`
- Date: `2026-04-21`
- Decision: A vessel may seal at most one Anky per UTC day, and only the current chamber may be sealed.
- Rationale: The daily right belongs to the vessel itself and moves forward one chamber at a time.

## D-004: Transfer Preserves Unused Rights But Not Duplicate Rights

- Status: `accepted`
- Date: `2026-04-21`
- Decision: If a vessel is transferred before its seal right is used on a given UTC day, the new steward may still use that day's right.
- Decision: If the vessel has already been used that day, transfer does not create a second daily right.
- Rationale: The right attaches to vessel-day state, not to a prior steward or wallet moment.

## D-005: Public Proof Must Respect Privacy And Memory Separation

- Status: `accepted`
- Date: `2026-04-21`
- Decision: Private written content remains offchain and does not transfer with the vessel.
- Decision: The public proof layer should store only minimum proof for seal validity, timing, stewardship at seal time, chamber/day identity, and reward eligibility.
- Decision: Vessel history and writer history must remain distinct.
- Rationale: The protocol should preserve the visible path of the vessel without pretending to custody the writer's inner archive.

## D-006: Reward Follows The Valid Seal

- Status: `accepted`
- Date: `2026-04-21`
- Decision: Reward eligibility follows a valid seal event, not mere vessel possession.
- Decision: A reward earned by a valid seal remains attached to the steward who sealed, even if the vessel is later transferred.
- Rationale: Incentives should reinforce the rite without making later ownership look like retroactive authorship.
