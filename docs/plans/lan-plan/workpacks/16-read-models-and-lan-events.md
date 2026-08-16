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

Current V0.9 code exposes typed scan summary, add-device route state, selected-device readiness, portal target filtering, persisted scan-history continuity, discovery and pairing history snapshots, typed host-subscription route snapshots, and a backend LAN runtime event-chain stream through `agent-service`. The backend stream, parent replay parser, desktop host-delivery decision, and portal listener/state edge are tested separately. No integration test drives a backend replay through a real Tauri `AppHandle` emission into the portal listener, so that complete delivery chain remains a Phase 1 expected-test gap before richer network-flow and physical/manual proof.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds a typed protocol/service proof layer for signed discovery rows, rejected discovery states, route custody safety, relay/cache unavailable states, and manual-required physical proof labels. That is proof that the read-model data spine exists for this slice. The portal now consumes that spine for selected LAN detail rows and Activity/Network diagnostics, and the current browser-proof artifact shows those Rust-backed surfaces on the real portal routes. The 2026-06-28 W16 packet adds service-backed LAN runtime event-chain transport; it is not proof of browser-side stream replay, broader network-flow evidence breadth, or physical household proof.

### 2026-06-23 bridge root plus 2026-06-28 integration reruns

- The parent Rust bridge no longer injects the hardcoded `Child Laptop` LAN sample for `devices`.
- `crates/lan-core` now builds an inventory-backed `LanBrowserAddDeviceReadModel` from the real Windows neighbor-table path with honest empty/manual-required states when no real LAN evidence exists.
- `crates/parent-runtime-core` now forwards Devices-route `agent-command-requested` actions into `agent-service` instead of swallowing them behind a passive reload fallback.
- `crates/parent-runtime-core` now serializes that Rust read model into the product host bridge snapshot used by Tauri.
- Passive Devices-route load now requests the browser-discovery scan snapshot with an allowed origin, so the real Tauri route enters `SCANNING` and then renders discovered LAN devices from runtime state.
- `crates/parent-runtime-core` now builds a typed `ParentSubscriptionEvent`, and `apps/parent-desktop/src-tauri` now exposes `parent_subscribe_route` / `parent_unsubscribe_route` so the product shell can receive host-owned route updates without a UI WebSocket.
- `apps/portal/src/host-bridge.ts` and `apps/portal/src/main.ts` now wire that Tauri event path into the live route shell, recycle subscriptions on route changes and product actions, apply subscribed `ParentSubscriptionEvent.events` into the real portal event buffer before the latest route snapshot, reject stale subscribed event batches or route snapshots once a newer Rust-backed view is already buffered, and keep the dev-web adapter on the same generated snapshot shape without pretending it is product-parity transport.
- The LAN discovery source matrix now admits `previous-scan-snapshot` as an implemented W15 source when restart-persisted scan history informs current hostname/label/platform continuity, instead of hiding that influence outside the typed read model.
- The Rust-owned LAN read model and parent host snapshot now carry `previous-scan-snapshot` as a canonical weak LAN source, so the UI is no longer waiting on a parallel TypeScript contract catalog to expose that state.
- Portal projection now reads `lanDiscoverySourceMatrix` from the Rust-backed route snapshot instead of reconstructing LAN truth inside the app.
- `apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx` now renders the LAN discovery source matrix inside the real Activity/Network drawer from that Rust-backed snapshot, including matrix generation time, visible workpack rows, implemented source proof, and weak-source fence rows such as `previous-scan-snapshot`.
- `apps/portal/src/ParentPortalRoute.tsx` now keeps product routes on the real `ParentPortalSvgSurface` instead of mounting the parallel React proof/diagnostic overlays on top of Browser-, Screen-, and app-game-related routes.
- `crates/parent-runtime-core` now returns empty `browserPanels` and empty screen-settings host responses on the product bridge until those routes have runtime-backed data instead of sample snapshots.
- This slice does not yet prove full `agent-service`/`lan-core` parity, browser-side replay consumption of the new backend stream, dev-web parity, signed-child/manual household proof, or physical multi-device household proof. The current product subscription path remains polling-backed route-snapshot delivery owned by Tauri; the backend LAN runtime event-chain stream is a separate service command/report path.

## Ownership boundary

```text
crates/schema owns canonical LAN read-model/event/bridge shapes.
crates/agent-protocol and crates/agent-service own protocol/service-backed read-model proof when selected.
crates/parent-runtime-core owns parent host snapshots and subscription events.
portal-domain/apps/portal own pure projection only.
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

- [x] Baseline read-model snapshots expose LAN pairing status, scan summary, add-device route state, selected-device readiness, portal target filtering, and signed-discovery relay/cache proof states.
- [x] Current branch labels relay/cache unavailable, physical household proof manual-required, and unsupported/non-controllable infrastructure states through typed service/domain state instead of prose-only docs.
- [x] Portal LAN selected-device details and Activity/Network diagnostics now consume the signed discovery relay spine, route custody, relay/cache, parent-decision, audit, route-requirement, and manual-proof read-model fields without inventing portal-only LAN state.
- [x] The add-device read model now carries a `lanDiscoverySourceMatrix` snapshot that maps all 25 LAN plan workpacks and concrete discovery sources into typed implemented, partial, manual-required, and not-implemented statuses.
- [x] Activity/Network diagnostics render source-matrix rows for LAN workpacks, implemented source proof, weak-source fencing, and matrix generation time from the service-backed read model.
- [x] Event types include interface changed, scan started, scan finished, evidence found, device found, device updated, online, offline, agent discovered, agent confirmed, and unknown detected.
- [x] Events include event id, timestamp, session/scan id where relevant, and affected device id for device events.
- [x] Read models are derived from canonical registry/evidence state.
- [x] Portal can replay snapshot plus events without duplicate cards.
- [x] Older subscribed route snapshots and event batches do not regress a newer Rust-backed portal view.
- [x] Empty, unavailable, degraded, stale, offline, agent-offline, and manual-required states are explicit.
- [x] Activity / Network diagnostics now show scan/evidence first-seen and last-seen timing, evidence expiry, signed adapter proof state, and policy-target history.
- [x] Visual snapshot proof exists for the current service-backed Devices/LAN, Activity/Network diagnostics, and Network policy target surfaces under `output/playwright/lan-source-matrix-plan-completion/`.
- [x] Replayable backend LAN event-stream transport beyond the current polling-backed subscription bridge exists as a scoped `agent-service` WebSocket command/report backed by Rust discovery-event history rows.
- [ ] Richer network-flow evidence breadth, browser-side replay consumption of the new backend stream, and physical/manual artifacts remain open.

## Acceptance And Proof

- Contract tests keep the LAN event row shape explicit across Rust-owned bridge surfaces.
- Focused Rust tests plus portal replay/consumer tests cover canonical event ordering, canonical-registry or evidence-derived history rows, explicit history-state serialization, host-bridge duplicate event-id suppression for subscribed LAN route events, signed-discovery relay/cache/manual-proof projection, evidence-window and policy-target diagnostics, portal buffering of repeated route-event batches without duplicate cards, rejection of stale subscribed route snapshots or event batches that would regress a newer Rust-backed route view, and the service-backed backend LAN runtime event-chain stream.
- The previously named `scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` aggregate runner is absent from the current repository. Existing focused seam tests remain useful, but they do not replace the missing aggregate verifier or the real `AppHandle`-to-listener regression.
- Playwright proof now covers the service-backed Devices route, Activity or Network evidence drawer, and policy-target LAN binding from the real Rust snapshot path. Replayable event-stream browser behavior remains a later proof obligation.
- Visual snapshot proof covers Devices/LAN, Activity/Network, and relevant policy-target routes so manual review can catch product gaps beyond the focused tests.

### Current slice evidence

- `output/lan-plan-proof/16-read-models-and-lan-events/01-rust-lan-read-model-validation.log`
- `output/lan-plan-proof/16-read-models-and-lan-events/02-rust-lan-read-model-note.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/03-product-route-overlay-removal-note.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/04-tauri-devices-auto-scan-proof.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/05-tauri-host-subscription-bridge.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/09-subscription-event-route-events.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/10-history-runtime-ui-reruns.md`
- `output/lan-plan-proof/16-read-models-and-lan-events/11-backend-lan-runtime-stream.md`
- `test-results/v0-9-lan-source-matrix-plan-completion/proof.json`
- `output/playwright/lan-source-matrix-plan-completion/browser-proof.json`

These artifacts currently prove the Rust-owned snapshot and history slice through the 2026-06-28 reruns:

- `crates/lan-core` inventory-backed LAN read-model generation compiles and unit-tests cleanly.
- `crates/lan-core/tests/unit/read_model.rs` derives discovery-event history rows from canonical registry/evidence state, carries explicit event ids and timestamps, and orders rows by occurrence time before linking the replay chain.
- `crates/parent-runtime-core` serializes that read model into the host bridge snapshot without the old hardcoded LAN sample and forwards Devices-route agent commands into `agent-service`.
- `apps/parent-desktop/src-tauri` still compiles against the updated bridge path.
- `apps/parent-desktop/src-tauri` now emits typed `ParentSubscriptionEvent` updates for subscribed routes, and the portal shell consumes them through Tauri events instead of a UI WebSocket transport.
- `ParentSubscriptionEvent` now carries the same typed `ParentRouteEventSnapshot` list that the action-result path already exposed, so subscribed product routes can consume Rust-owned event ids, correlation ids, timestamps, and peer metadata without inventing portal-side event identity.
- `apps/portal/src/main.ts` now applies those subscribed `ParentSubscriptionEvent.events` through the same portal event-buffer path used by command responses, so subscribed LAN route updates do not silently drop Rust-owned event history while refreshing the latest snapshot.
- `ParentSubscriptionEvent` now dedupes duplicate subscribed route events by `eventId` at the Rust host bridge, keeping the latest payload-bearing occurrence so snapshot-plus-event replay does not double-emit the same LAN route event identity from one subscription poll.
- `apps/portal/tests/portal/portal-state-target.test.ts` now proves that replaying the same host-bridge event batch twice preserves the newest-first buffer and avoids duplicate portal cards.
- `apps/portal/tests/portal/portal-state-target.test.ts` now also proves that a stale subscribed batch cannot overwrite a newer Rust-backed `/devices` route snapshot or prepend older LAN event history back to the top of the portal buffer.
- `ParentSubscriptionEvent` keeps the LAN diagnostics and discovery-history surface intact, preserves explicit `empty`, `manual-required`, `unavailable`, and `degraded` history labels, and projects stale selected-device metadata as `stale` from Rust-owned route metadata.
- Focused 2026-06-28 reruns now also prove persisted scan-history sidecars, signed-child passive beacon history rows, Rust contract preservation of `lanAddDeviceReadModel`, Devices-route passive load and signed-child observe forwarding, and portal LAN snapshot consumption in the Network evidence drawer and activity intent surfaces.
- Focused 2026-06-28 W16 reruns now prove the protocol/service-backed backend LAN runtime event-chain stream command and report shape: `agent.lan.runtime.event-chain.stream.get` reads the Rust LAN discovery-event history and returns `agent.lan.runtime.event-chain.stream.reported` with replayable `eventType`, `eventRef`, and row payload entries.
- `crates/agent-service/tests/unit/lan_pairing_browser_add_device_state.rs` and `apps/portal/tests/unit/activity-ui-intent.test.ts` now prove the Rust-owned add-device snapshot and portal consumers preserve selected-device readiness, relay/cache unavailable state, manual-proof state, route custody, parent decision, router/infrastructure visibility, and unsupported non-child boundaries without UI-side invention.
- Product Browser and Settings routes no longer mount the parallel React overlay shell when served from the local portal bundle.
- The real Tauri Devices route now auto-enters `SCANNING` and renders the discovered LAN inventory without a manual scan click after route entry.
- The current visual proof root under `output/playwright/lan-source-matrix-plan-completion/` captures the live Devices LAN surface, Activity/Network evidence diagnostics, and policy-target binding from the real Rust snapshot path.

These artifacts do not prove:

- full `agent-service` and `lan-core` parity
- browser-side replay consumption of the new backend LAN runtime event-chain stream
- broader network-flow evidence breadth
- dev-web adapter parity
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
