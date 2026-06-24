# 16 Read Models And LAN Events

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `16 Read Models And LAN Events`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 proof exposes typed scan summary, add-device route state, selected device readiness, portal target filtering, and a development UI that can show a LAN device grid, Activity tabs, and Network evidence summary fields. Production discovery needs a replayable LAN event stream plus read models that all portal surfaces can consume.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds a typed protocol/service proof layer for signed discovery rows, rejected discovery states, route custody safety, relay/cache unavailable states, and manual-required physical proof labels. That is proof that the read-model data spine exists for this slice. The portal now consumes that spine for selected LAN detail rows and Activity/Network diagnostics. That is not proof that the full replayable event stream or visual/browser proof is complete.

### 2026-06-23 Rust bridge slice

- The parent Rust bridge no longer injects the hardcoded `Child Laptop` LAN sample for `devices`.
- `crates/lan-core` now builds an inventory-backed `LanBrowserAddDeviceReadModel` from the real Windows neighbor-table path with honest empty/manual-required states when no real LAN evidence exists.
- `crates/parent-runtime-core` now forwards Devices-route `agent-command-requested` actions into `agent-service` instead of swallowing them behind a passive reload fallback.
- `crates/parent-runtime-core` now serializes that Rust read model into the product host bridge snapshot used by Tauri.
- Passive Devices-route load now requests the browser-discovery scan snapshot with an allowed origin, so the real Tauri route enters `SCANNING` and then renders discovered LAN devices from runtime state.
- `crates/parent-runtime-core` now builds a typed `ParentSubscriptionEvent`, and `apps/parent-desktop/src-tauri` now exposes `parent_subscribe_route` / `parent_unsubscribe_route` so the product shell can receive host-owned route updates without a UI WebSocket.
- `apps/portal/src/host-bridge.ts` and `apps/portal/src/main.ts` now wire that Tauri event path into the live route shell, recycle subscriptions on route changes and product actions, and keep the dev-web adapter on the same typed subscription contract without pretending it is product-parity transport.
- The LAN discovery source matrix now admits `previous-scan-snapshot` as an implemented W15 source when restart-persisted scan history informs current hostname/label/platform continuity, instead of hiding that influence outside the typed read model.
- `packages/schema-domain/src/lan-source-matrix.ts` now treats `previous-scan-snapshot` as a canonical weak LAN source instead of leaving the Rust read model ahead of the TypeScript contract catalog.
- `packages/portal-domain/src/live-activity-lan-add-device.ts` now exposes `lanDiscoverySourceMatrix` on the typed portal add-device read model, and `apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx` now projects the Activity/Network matrix from that typed field instead of walking raw records inside the app.
- `apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx` now renders the LAN discovery source matrix inside the real Activity/Network drawer, including matrix generation time, visible workpack rows, implemented source proof, and weak-source fence rows such as `previous-scan-snapshot`.
- `apps/portal/src/ParentPortalRoute.tsx` now keeps product routes on the real `ParentPortalSvgSurface` instead of mounting the parallel React proof/diagnostic overlays on top of Browser-, Screen-, and app-game-related routes.
- `crates/parent-runtime-core` now returns empty `browserPanels` and empty screen-settings host responses on the product bridge until those routes have runtime-backed data instead of sample snapshots.
- This slice does not yet prove full `agent-service`/`lan-core` parity, canonical replayable LAN events, dev-web parity, signed relay/cache rows, portal screenshot proof, or physical multi-device household proof. The current product subscription path is still polling-backed route-snapshot delivery owned by Tauri, not final backend event-stream replay.

## Ownership boundary

```text
schema-domain owns canonical LAN read-model/event shapes.
agent-protocol and agent-service own protocol/service-backed read-model proof when selected.
portal-domain/apps/portal own projection only.
eventing-plan owns local event bus semantics only.
lan-plan owns the LAN read-model/event proof boundary and no-claim routing.
```

## Where We Want To Be

LAN events and read models expose scan state, discovered devices, trusted devices, route state, controller/observer authority, child-agent presence, manual proof requirements, stale/offline state, evidence details, and Activity/Network diagnostics that make service behavior visually inspectable.

## Proof separation

The selected proof must distinguish these states instead of collapsing them:

```text
read_model_field_exists
service_backed_state_exists
portal_projection_exists
replayable_event_stream_exists
duplicate_prevention_exists
stale_event_rejection_exists
visual_snapshot_exists
physical_household_proof_exists
manual_required_state
no_claim
```

A field existing in a read model is not proof of replayable event stream behavior. Portal projection is not LAN truth proof. Eventing local bus proof is not LAN transport proof. Visual snapshots are not service/runtime proof unless backed by the selected service proof.

## Requirement Checklist

- [ ] Baseline read-model snapshots expose LAN pairing status, scan summary, add-device route state, selected-device readiness, portal target filtering, and signed-discovery relay/cache proof states.
- [ ] Current branch labels relay/cache unavailable, physical household proof manual-required, and unsupported/non-controllable infrastructure states through typed service/domain state instead of prose-only docs.
- [ ] Portal LAN selected-device details and Activity/Network diagnostics now consume the signed discovery relay spine, route custody, relay/cache, parent-decision, audit, route-requirement, and manual-proof read-model fields without inventing portal-only LAN state.
- [x] The add-device read model now carries a `lanDiscoverySourceMatrix` snapshot that maps all 20 LAN plan workpacks and concrete discovery sources into typed implemented, partial, manual-required, and not-implemented statuses.
- [x] Activity/Network diagnostics render source-matrix rows for LAN workpacks, implemented source proof, weak-source fencing, and matrix generation time from the service-backed read model.
- [ ] Event types include interface changed, scan started, scan finished, evidence found, device found, device updated, online, offline, agent discovered, agent confirmed, and unknown detected.
- [ ] Events include event id, timestamp, session/scan id where relevant, and affected device id for device events.
- [ ] Read models are derived from canonical registry/evidence state.
- [ ] Portal can replay snapshot plus events without duplicate cards.
- [ ] Empty, unavailable, degraded, stale, offline, agent-offline, and manual-required states are explicit.
- [ ] Activity / Network diagnostics now show scan/evidence first-seen and last-seen timing, evidence expiry, signed adapter proof state, and policy-target history.
- [ ] Visual snapshot proof exists for the current service-backed Devices/LAN, Activity/Network diagnostics, and Network policy target surfaces under `output/playwright/lan-source-matrix-plan-completion/`.
- [ ] Replayable scan sessions, full pairing/heartbeat event history, richer network-flow evidence remain open.

## Acceptance And Proof

- Contract tests protect the LAN event stream shape.
- Service tests cover event ordering and duplicate event id rejection.
- `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` proves the source-matrix read-model field is preserved across TypeScript contracts, Rust protocol, Rust service state, and portal render data.
- Playwright contract-fixture UI tests cover empty dashboard, progressive scan, evidence panel, assignment, confirmed badge, offline state, and malicious hostname rendering. Later real-backend tests cover service-backed routes.
- Visual snapshot proof covers Devices/LAN, Activity/Network, and relevant policy target routes so manual review can catch product gaps beyond tests.

### Current slice evidence

- `output/lan-plan-proof/16-read-models-and-lan-events/01-rust-lan-read-model-validation.log`
- `output/lan-plan-proof/16-read-models-and-lan-events/02-rust-lan-read-model-note.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/03-product-route-overlay-removal-note.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/04-tauri-devices-auto-scan-proof.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/05-tauri-host-subscription-bridge.md`

These artifacts prove only the 2026-06-23 parent Rust bridge slice:

- `crates/lan-core` inventory-backed LAN read-model generation compiles and unit-tests cleanly.
- `crates/parent-runtime-core` serializes that read model into the host bridge snapshot without the old hardcoded LAN sample and forwards Devices-route agent commands into `agent-service`.
- `apps/parent-desktop/src-tauri` still compiles against the updated bridge path.
- `apps/parent-desktop/src-tauri` now emits typed `ParentSubscriptionEvent` updates for subscribed routes, and the portal shell consumes them through Tauri events instead of a UI WebSocket transport.
- Product Browser and Settings routes no longer mount the parallel React overlay shell when served from the local portal bundle.
- The real Tauri Devices route now auto-enters `SCANNING` and renders the discovered LAN inventory without a manual scan click after route entry.

These artifacts do not prove:

- full `agent-service` and `lan-core` parity
- replayable LAN event stream behavior beyond polling-backed route snapshot updates
- dev-web adapter parity
- portal screenshot/manual review proof
- physical two-device/router/firewall proof

## Required proof fields

The selected proof must name, at minimum:

```text
read_model_ref
service_state_ref
portal_projection_ref
event_stream_ref
duplicate_prevention_state
stale_event_state
replay_state
manual_required_state
visual_snapshot_state
physical_household_state
no_claim
```

## Failure conditions

- Read-model field presence is used as replayable event-stream proof.
- Portal projection is used as LAN truth proof.
- Eventing local-bus proof is used as LAN transport proof.
- Visual snapshot proof is used as service/runtime proof without the selected backing service proof.
- Single-machine proof is used as physical household proof.

## Parallel Ownership Notes

Portal UI work for this LAN plan is part of codex-b ownership. The portal must not invent LAN state or own merge/registry truth; it renders service-backed state from the canonical household device/read-model path.
