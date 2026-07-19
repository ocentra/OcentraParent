# WP25 LAN Replay Consumer Rollout Truth

Date: 2026-07-19

State: `partial/manual`

## Proven locally

- The parent Rust agent-service client requests the canonical LAN runtime
  event-chain stream through the localhost transport.
- Rust binds the response to the exact LAN replay command message ID and
  validates the response schema, event and event ID, RFC3339 timestamp,
  source/target peers and roles, severity, and no-snapshot acceptance shape.
- Rust validates generated/latest metadata, count/state consistency, row
  schema, canonical event type and reference, unique IDs, RFC3339 instant
  order across timezone offsets, and the previous-event chain before producing
  `ParentRouteEventSnapshot` values. Empty history accepts canonical
  metadata-only interface-change and scan-lifecycle rows while rejecting
  material discovery rows; ready history requires at least one material row;
  manual-required history rejects nonempty streams. Unavailable takes
  precedence before row inspection, and degraded and agent-offline precedence
  are handled explicitly.
- Derived replay rows preserve the already-validated envelope source, target,
  roles, and severity; blank or spoofed route identity is rejected rather than
  replaced with manufactured defaults.
- Valid entries reach `ParentSubscriptionEvent` in producer order. Live status
  events are merged last so they retain authority on an event-ID collision.
- The separately loaded status and replay histories must agree on history state,
  latest event ID, and latest observed time. State, ID, or time mutation between
  reads rejects the replay batch.
- Wrong-command correlation, invalid/spoofed envelopes, missing or mismatched
  nonempty latest-row metadata, invalid timestamps, duplicate rows,
  stale/out-of-order instants, broken chains, inconsistent counts, and
  malformed replay batches fail closed without erasing the live LAN status
  snapshot.
- Replay failure adds one host-owned warning event with a unique safe event ID,
  valid RFC3339 timestamp, fixed service-to-portal peer identity and roles,
  warning severity, and no correlation, payload, snapshot, or rejected raw
  value.
- The desktop host delivery decision emits unseen event IDs even when the route
  snapshot is stable and suppresses already delivered IDs.
- The isolated portal state edge binds only replay rows to the Rust snapshot,
  accepts a realistic later live-status event, rejects nonempty replay rows with
  missing, invalid, or mismatched history metadata, buffers the safe warning
  once, and keeps the identical newest 128 rows when a 129-plus full history is
  repeated. It does not prove that the real Tauri emitter delivered a backend
  replay batch to that listener.
- Recent and stale persisted scan caches preserve identical previous/current
  snapshot context for localhost status and local-network replay-stream reads;
  a restart-backed stream no longer fabricates scan-delta lifecycle/device rows.
- Explicit offline and manual-required read-model states remain visible when
  the replay batch is empty or rejected.

## Evidence

- Exact focused validation commands and counts:
  `16-validation-commands.log`
- Rust consumer integration tests:
  `crates/parent-runtime-core/tests/integration/parent_ui_bridge/lan_replay_tests.rs`
- Desktop host-decision test:
  `apps/parent-desktop/src-tauri/tests/unit/desktop_shell.rs`
- Isolated portal edge test:
  `apps/portal/tests/portal/portal-state-target.test.ts`

## Explicit non-claims

This packet does not prove physical two-device or multi-device topology,
router/firewall behavior, signed install artifacts, restart/cross-session
physical replay, mobile platform behavior, or any other manual LAN artifact. It
also does not prove the complete backend-to-Tauri-emitter-to-portal-listener
chain or manual runtime behavior; that requires a real host emission observed
at the portal subscription edge plus the separately owned manual artifacts.
WP25 therefore remains `partial/manual`; this packet does not support broad
PR-ready or DONE.
