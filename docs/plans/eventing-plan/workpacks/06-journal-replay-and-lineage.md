# WP06 Journal Replay And Lineage

Scope: prove append/replay behavior, lineage compatibility, event topology, journal modes, and replay safety.

Source rows: `05-implementation-workpacks.md` rows 36-41 and 69-78.

Read next:

- `../05-implementation-workpacks.md` rows 36-41 and 69-78 only
- `../07-lineage-preservation-and-migration-safety.md`
- `../04-tests-proof-and-validation.md`
- `../TEST_PROOF_EXPECTATIONS.md`

Expected outcome:

- Journal trait, append format, replay cursor, replay filters, and replay safety gate are explicit.
- Journal-before-dispatch and journal-after-dispatch modes are distinguished.
- Lineage from earlier Unity/TypeScript semantics is documented with accepted differences.
- Topology manifest identifies publishers, subscribers, one-sided events, family variants, orphan publishers, and no-subscriber cases.
- Runtime ownership, shutdown, drain, test clear, duplicate subscription, and journaling allowlist behavior are proved.

Expected tests/proof:

- `eventing.journal.append-roundtrip`
- `eventing.journal.corruption-negative`
- `eventing.replay.cursor-filter`
- `eventing.replay.projection-only-safety`
- `eventing.lineage.compatibility-suite`
- `eventing.topology.manifest`
- `eventing.runtime.no-global-singleton`
- Proof includes fixture path, replay transcript, corruption case, migration/rollback note, and topology artifact.

Failure conditions:

- Do not claim production retention/delete behavior; data custody owns that.
- Do not claim remote replication; relay/cloud work owns that.
- Do not replay side effects unless replay safety gate proves projection-only behavior.
