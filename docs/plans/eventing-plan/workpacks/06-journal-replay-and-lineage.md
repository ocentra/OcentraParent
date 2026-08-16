# WP06 Journal Replay And Lineage

Scope: prove append/replay behavior, lineage compatibility, event topology, journal modes, and replay safety.

Current state: code correction drafted; tests/validation/proof deferred. This
packet is the selectable generic journal prerequisite for enforcement WP11. The
retained WP06 durable manifest ties the focused crate evidence to the
topology/journal proof and the production typed, redacted enforcement-audit
journal handoff. The handoff records before-action and final audit summaries
before their corresponding activity audit writes; it does not implement or
prove enforcement dispatch. The current source audit also found and corrected
the NDJSON recovery caller's narrow append-lock visibility boundary; the
retained proof predates that correction and is not refreshed by this slice.

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
- Retain `docs/proof/eventing-plan/wp06-00-enforcement-wp11-handoff.md` with the typed generic journal/replay/idempotency handoff. It is an Eventing-owned handoff record, not an enforcement implementation or proof of dispatch.

Failure conditions:

- Do not claim production retention/delete behavior; data custody owns that.
- Do not claim remote replication; relay/cloud work owns that.
- Do not replay side effects unless replay safety gate proves projection-only behavior.
- Do not describe a blocker or existing crate test as an enforcement scheduling satisfaction; without the required handoff artifact, WP11 and WP04 remain blocked/manual-required.

Proof retained:

- `docs/proof/eventing-plan/wp06-00-enforcement-wp11-handoff.md`
- `docs/proof/eventing-plan/wp06-01-journal-replay-proof.md`
- `docs/proof/eventing-plan/wp06-02-topology-lineage-proof.md`
- `docs/proof/eventing-plan/wp06-16-validation-commands.md`

The generic prerequisite is locally evidenced. Enforcement WP11/WP04 still
own adapter authority, action execution, authorization, audit, rollback, and
platform-side-effect proof.
