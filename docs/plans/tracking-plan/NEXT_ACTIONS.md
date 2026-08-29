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

1. **Child WP10 then Child WP05 Android owner chain**: finish the reviewed
   trusted startup handoff, then the shipped Android package, JNI bridge,
   service lifecycle, and real platform-test boundary. Tracking must consume
   this owner; it must not add a parallel Android service or dead handoff.
2. **WP40 trusted runtime ingress and journal composition**: establish the
   missing shipped child/service owner for trusted tracking ingress, canonical
   event-to-journal mapping, durable key/path configuration, startup recovery,
   and idempotent ActivityStore projection. WP40 is a route/workpack only;
   implementation and tests have not started.
3. **WP08/WP10, then WP09 Android tracking adapters**: foreground location and
   battery/connectivity/status consume reviewed Child WP05. Background
   location/geofence consumes reviewed Child WP05 plus reviewed WP40 ingress.
4. **WP37 durable journal replay/projection**: connect the existing WP35/WP36
   tracking event flows to durable append, restart replay, and idempotent SQLite
   projection. This remains the highest unblocker, but is currently blocked at
   composition: `TrackingRuntimeEventFlow::new`/the parent check-in flow use an
   in-memory `EventBus`, and `ActivityStore::ingest_journal` accepts separate
   `ActivityEvent` journal lines. A future owner must provide the trusted
   event mapping, durable journal key/path configuration, startup replay, and
   projection wiring after WP40 exists; do not add a dead constructor or
   synthetic adapter here.
5. **WP38 then WP27 notification/escalation**: add the durable notification
   outbox/receipt boundary and escalation/quiet-hours/ack timer lifecycle.
6. **WP22 then WP07 persistence/custody**: replace the in-memory place store and
   transformation-only retention path with durable, restart-safe execution.
7. **WP11-WP12 iOS adapters**: foreground/background/region lifecycle code
   with real platform tests after its package/runtime owner is routed.
8. **WP20 and WP24 providers**: concrete POI provider and selected AI provider
   routing with redaction, timeout, retry, and receipt tests.
9. **WP28-WP30 and WP39 product composition**: durable live/missing-device
   lifecycle plus complete parent/child UI and event-to-portal restart proof.
10. **WP33 verifier restoration**: restore or deliberately replace the absent
   tracked aggregate `scripts/test/tracking-*.mjs` proof contract after code and
   focused tests are green.

## Current sharp blockers

- `TrackingRuntimeEventFlow` owns a fresh in-memory `EventBus`; it does not
  append/replay a durable tracking journal.
- The SQLite tracking read model is not fed by that live cascade; the existing
  `agent-service` tracking report only reads the independently populated
  `ActivityStore`.
- No shipped tracking-event-to-`ActivityEvent` mapping or durable journal
  composition owner exists. WP40 is now the explicit dependency blocker;
  WP37 is not a legal code-only slice until WP40 is implemented.
- No production Android/iOS tracking adapter exists. Android acquisition is
  blocked on the reviewed Child WP10 -> Child WP05 package/bridge chain; WP09
  also waits for reviewed WP40 ingress.
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
