# App Surface Notes

The iOS app is the canonical entrance to Sojourn 9.

This directory is for implementation notes that preserve the constitutional surface without trying to replace the constitution itself. Any app-facing spec written here should stay centered on the canonical surfaces named in `constitution/SOJOURN_9.md`:

- Sojourn Map
- Writing Chamber
- Return Chamber
- Sealed Chamber View
- Codex
- Vessel

Those notes should preserve these constitutional behaviors:

- today's chamber is the first action
- the active vessel must be clear whenever a wallet holds more than one vessel
- silence comes before optional openings
- chamber states remain legible as future, today/open, in progress, sealed, or missed
- writer history stays private even when vessel history remains visible
