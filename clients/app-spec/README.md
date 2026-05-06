# App Surface Notes

The iOS app is the canonical entrance to Sojourn 9.
This directory holds notes for that surface.

These notes do not replace the constitution.
They should stay centered on the canonical surfaces named in `constitution/SOJOURN_9.md`:

- Sojourn Map
- Writing Chamber
- Return Chamber
- Sealed Chamber View
- Codex
- Vessel

Any app-facing spec here should preserve:

- while supply remains, the app invites an unclaimed wallet to claim its vessel
- claim is a deliberate app action, conceptually a swipe-to-claim gesture
- a valid vessel is a soulbound cNFT in the canonical Sojourn 9 collection of 3,456 vessels
- a wallet may steward at most one valid vessel
- once claimed, the vessel remains bound to that wallet for the duration of the season
- after claim, today's chamber is the first action
- the canonical app should not introduce selected-vessel ambiguity
- the device computes the `.anky` hash commitment from the exact raw `.anky` string and sends only that commitment to the program
- the chain records commitment, not content
- silence comes before optional openings
- chamber states remain legible as future, today/open, in progress, sealed, or missed
- writer history stays private even when vessel history remains visible

If implementation scaffolding still exposes `selected_vessel_*` inputs, treat that as a transitional program boundary rather than app-level season law.
