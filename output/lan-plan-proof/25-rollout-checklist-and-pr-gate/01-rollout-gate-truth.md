# WP25 LAN Replay Consumer Rollout Truth

Date: 2026-07-18

State: `partial/manual`

## Proven locally

- The parent Rust agent-service client requests the canonical LAN runtime
  event-chain stream through the localhost transport.
- Rust validates the stream envelope, count/state metadata, row schema,
  canonical event type and reference, unique IDs, chronological order, and
  previous-event chain before producing `ParentRouteEventSnapshot` values.
- Valid entries reach `ParentSubscriptionEvent` in producer order. Live status
  events are merged last so they retain authority on an event-ID collision.
- Duplicate, stale/out-of-order, broken-chain, inconsistent, and malformed
  replay batches fail closed without erasing the live LAN status snapshot.
- The portal state edge buffers the Rust-supplied events newest first without
  acquiring LAN replay business logic.
- Explicit offline and manual-required read-model states remain visible when
  the replay batch is empty or rejected.

## Evidence

- Exact focused validation commands and counts:
  `16-validation-commands.log`
- Rust consumer integration tests:
  `crates/parent-runtime-core/tests/integration/parent_ui_bridge/lan_replay_tests.rs`
- Portal edge test:
  `apps/portal/tests/portal/portal-state-target.test.ts`

## Explicit non-claims

This packet does not prove physical two-device or multi-device topology,
router/firewall behavior, signed install artifacts, restart/cross-session
physical replay, mobile platform behavior, or any other manual LAN artifact.
WP25 therefore remains `partial/manual`; this packet does not support broad
PR-ready or DONE.
