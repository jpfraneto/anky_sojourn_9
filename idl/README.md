# IDL

This directory holds reviewed public IDL snapshots for the Sojourn 9 proof layer.

An IDL committed here should describe only the onchain public-proof surface. It is not the constitution, and it is not the canonical app surface.

Expected workflow:

1. build the program with `anchor build`
2. copy the generated IDL from `program/target/idl/sojourn_9.json`
3. review the snapshot against `constitution/SOJOURN_9.md` and the current Rust source
4. commit the reviewed snapshot here
5. update deployment manifests to point at the committed IDL path plus the active constitutional reference

Helper command:

```bash
npm run idl:sync
```

No reviewed IDL snapshot has been committed yet.
