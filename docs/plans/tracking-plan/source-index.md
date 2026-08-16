# Tracking Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Use only the named package/crate path after selecting a workpack.
> Proves: current source routing only, not implementation acceptance or product readiness.

<!-- /agent-capsule -->

Reconciled against the live checkout on 2026-08-15. The exact per-workpack
source/test map and implementation gaps are in [CODE_AUDIT.md](CODE_AUDIT.md).

## Product and planning inputs

- `docs/features/location-geofence-device-status.md`
- `docs/expectations/location-geofence.md`
- `docs/expectations/platforms.md`
- `docs/expectations/notifications.md`
- `docs/expectations/policy.md`
- `docs/expectations/ai.md`
- `docs/expectations/data-custody.md`
- `docs/plans/tracking-plan/workpacks/tracking-control-settings-inventory.md`
- `docs/plans/tracking-plan/workpacks/device-location-tracking-capability-guide.md`
- `docs/plans/tracking-plan/workpacks/device-location-tracking-schema-proposal.md`

These are requirements and routing inputs. They are not implementation proof.

## Current Rust owners

| Owner | Current responsibility |
| --- | --- |
| `crates/schema/src/tracking_event_contracts.rs` | Cross-family tracking event registry and validation. |
| `crates/agent-protocol/src/tracking/` | Typed tracking identifiers, runtime/config events, read models, retention commands, and `DomainEvent` identity. |
| `crates/tracking-core/` | Location validation, status/capability evaluation, geofence and expected-place decisions, acknowledgement, check-in helpers, nearby-place boundaries, AI evidence validation, alerting, live/missing-device decisions, retention transforms, and SQLite read-model queries. |
| `crates/parent-runtime-core/src/tracking_config_update_flow.rs` | Parent-authorized tracking config event flow and audit/read-model hops. |
| `crates/parent-runtime-core/src/tracking_child_check_in_request_flow.rs` | Parent check-in request flow. |
| `crates/child-runtime/src/tracking_config_update_flow.rs` | Child config apply/persist response flow. |
| `crates/child-runtime/src/tracking_runtime_flow.rs` | Process-local tracking detection/event cascade. |
| `crates/policy-control-core/src/policy_compiler.rs` | Canonical domain policy compilation for Tracking. |
| `crates/child-policy-core/src/tracking_policy.rs` | Child-local nearby/expected-place policy evaluation. |
| `crates/child-ai-core/src/tracking_boundary.rs` | Tracking AI request validation/classification boundary. |
| `crates/child-notification-core/src/tracking_notification.rs` | Policy-violation to parent-notification intent conversion. |
| `crates/agent-core/src/tracking/mod.rs` | ActivityStore-backed tracking read-model facade. |
| `crates/agent-service/src/activity_api.rs` and tracking WebSocket files | Service read-model and retention-write transport seams. |

## Parent presentation owners

- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot/tracking.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/tracking_panel.rs`
- `packages/portal-domain/src/tracking-status-panel.ts`
- `packages/portal-domain/src/tracking-status-panel-helpers*.ts`
- `apps/portal/src/TrackingStatusRoutePanel.tsx`
- `apps/portal/src/tracking-status-route-panel-body.tsx`

TypeScript in these paths is presentation/generated-edge code. It does not own
tracking contracts, policy, evidence, or runtime decisions.

## Current tests

- `crates/schema/tests/contract/tracking_event_contracts.rs`
- `crates/agent-protocol/tests/contract/tracking_*.rs`
- `crates/tracking-core/tests/`
- `crates/parent-runtime-core/tests/unit/tracking_*.rs`
- `crates/child-runtime/tests/unit/tracking_*.rs`
- `crates/child-runtime/tests/integration/tracking_runtime_flow_intent.rs`
- `crates/child-policy-core/tests/unit/tracking_policy.rs`
- `crates/child-ai-core/tests/security/tracking_boundary.rs`
- `crates/child-notification-core/tests/contract/tracking_notification.rs`
- `crates/agent-core/tests/unit/tracking_read_model.rs`
- `crates/agent-service/tests/unit/tracking_*.rs`
- `packages/portal-domain/tests/unit/tracking-*.ts`
- `apps/portal/tests/unit/tracking-status-panel.test.ts`
- `apps/portal/tests/e2e/tracking-hosted-ui-proof.spec.ts`

## Removed or stale ownership references

- `packages/tracking-domain` does not exist in the current checkout. Do not route
  new contracts or behavior there.
- The prior `scripts/test/tracking-*.mjs` aggregate proof suite does not exist in
  the current checkout. Workpacks and snapshots that name those scripts are
  stale until a tracked executable verifier is restored or the proof contract
  is deliberately replaced.
- `packages/schema-domain` contains transitional/generated tracking-control
  catalog presentation support. It is not canonical product authority.
- Historical TypeScript-like schema proposals remain reference material; Rust
  protocol/schema owners above are current authority.

## Verified missing source families

- Android Fused Location, foreground service, background permission, system
  geofence, and battery/connectivity production adapters.
- iOS Core Location foreground, Always/region, significant-change, relaunch,
  and authorization adapters.
- Desktop presence-hint collectors with provenance/no-precise-location tests.
- Concrete Google Places, MapKit, or OpenStreetMap adapter.
- Durable parent-defined place database.
- Selected AI provider route/execution lifecycle for tracking.
- Durable escalation engine, quiet-hours timer, notification outbox, provider
  receipt, retry, and dead-letter lifecycle.
- Durable temporary-live and missing-device runtime composition.
- Durable tracking event journal replay into the SQLite read model.
- End-to-end event-to-portal restart proof and child product UI.

## Boundary rules

- LAN/IP/Wi-Fi presence is never precise child-location proof.
- Nearby-place results preserve radius, provider, ambiguity, confidence, and
  evidence refs; they never prove that a child is inside a place.
- AI results are evidence only. Policy retains action authority.
- Notification intent is not provider delivery or receipt proof.
- Location remains local/LAN-first unless an explicitly authorized remote path
  is selected.
- Real Android/iOS background claims require physical-device lifecycle proof.
- Shared eventing, journal, replay, custody, notification, AI provider, and
  policy mechanics remain owned by their neutral/adjacent crates.

## Audit route

1. Read `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and `WORKPACK_INDEX.md`.
2. Open one workpack.
3. Use this file to select the current owner and [CODE_AUDIT.md](CODE_AUDIT.md)
   for the last reviewed source/test status.
4. Re-run the focused tests and Enforcer checks for the touched slice.
5. Generate proof only after code and tests are green.
