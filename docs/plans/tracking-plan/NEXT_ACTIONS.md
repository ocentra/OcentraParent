# Tracking Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Next Actions`
> Kind: resume queue and highest-open work.
> Proves: routing only, not authorization or completion.

<!-- /agent-capsule -->

Use [CODE_AUDIT.md](CODE_AUDIT.md) for the 2026-08-15 source/test result and
`WORKPACK_INDEX.md` to open one selected workpack.

## Dependency-first order

1. **WP37 durable journal replay/projection**: connect the existing WP35/WP36
   tracking event flows to durable append, restart replay, and idempotent SQLite
   projection. This remains the highest unblocker, but is currently blocked at
   composition: `TrackingRuntimeEventFlow::new`/the parent check-in flow use an
   in-memory `EventBus`, and `ActivityStore::ingest_journal` accepts separate
   `ActivityEvent` journal lines. A future owner must provide the trusted
   event mapping, durable journal key/path configuration, startup replay, and
   projection wiring; do not add a dead constructor or synthetic adapter here.
2. **WP38 then WP27 notification/escalation**: add the durable notification
   outbox/receipt boundary and escalation/quiet-hours/ack timer lifecycle.
3. **WP22 then WP07 persistence/custody**: replace the in-memory place store and
   transformation-only retention path with durable, restart-safe execution.
4. **WP08-WP12 platform adapters**: Android foreground/background/geofence and
   iOS foreground/background/region lifecycle code with real platform tests.
5. **WP20 and WP24 providers**: concrete POI provider and selected AI provider
   routing with redaction, timeout, retry, and receipt tests.
6. **WP28-WP30 and WP39 product composition**: durable live/missing-device
   lifecycle plus complete parent/child UI and event-to-portal restart proof.
7. **WP33 verifier restoration**: restore or deliberately replace the absent
   tracked aggregate `scripts/test/tracking-*.mjs` proof contract after code and
   focused tests are green.

## Current sharp blockers

- `TrackingRuntimeEventFlow` owns a fresh in-memory `EventBus`; it does not
  append/replay a durable tracking journal.
- The SQLite tracking read model is not fed by that live cascade; the existing
  `agent-service` tracking report only reads the independently populated
  `ActivityStore`.
- No shipped tracking-event-to-`ActivityEvent` mapping or durable journal
  composition owner exists, so WP37 is a dependency blocker rather than a
  legal code-only slice in this pass.
- No production Android/iOS tracking adapter exists.
- No durable notification/escalation lifecycle exists.
- No concrete POI or tracking AI provider execution route exists.
- Old `tracking-domain` and tracking proof-script routes are stale/absent.

## Slice rules

- One workpack per implementation slice.
- Claim exact files through Enforcer before edits.
- Preserve Rust-first contract/behavior ownership and presentation-only TS.
- Run focused tests and Enforcer in Phase 2; do not start with broad CI.
- Regenerate proof only after the code/test slice is green.
- Update the workpack, this queue, `PLAN_STATE.md`, and the engineering graph
  when findings or status change.
