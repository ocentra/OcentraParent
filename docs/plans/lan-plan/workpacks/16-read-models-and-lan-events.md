# 16 Read Models And LAN Events

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 proof exposes typed scan summary, add-device route state, selected
device readiness, portal target filtering, and a development UI that can show a
LAN device grid, Activity tabs, and Network evidence summary fields. Production
discovery needs a replayable LAN event stream plus read models that all portal
surfaces can consume.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds a typed
protocol/service proof layer for signed discovery rows, rejected discovery
states, route custody safety, relay/cache unavailable states, and
manual-required physical proof labels. That is proof that the read-model data
spine exists for this slice. The portal now consumes that spine for selected
LAN detail rows and Activity/Network diagnostics. That is not proof that the
full replayable event stream or visual/browser proof is complete.

## Where We Want To Be

LAN events and read models expose scan state, discovered devices, trusted
devices, route state, controller/observer authority, child-agent presence,
manual proof requirements, stale/offline state, evidence details, and
Activity/Network diagnostics that make service behavior visually inspectable.

## Requirement Checklist

- [x] Baseline read-model snapshots expose LAN pairing status, scan summary,
      add-device route state, selected-device readiness, portal target
      filtering, and signed-discovery relay/cache proof states.
- [x] Current branch labels relay/cache unavailable, physical household proof
      manual-required, and unsupported/non-controllable infrastructure states
      through typed service/domain state instead of prose-only docs.
- [x] Portal LAN selected-device details and Activity/Network diagnostics now
      consume the signed discovery relay spine, route custody, relay/cache,
      parent-decision, audit, route-requirement, and manual-proof read-model
      fields without inventing portal-only LAN state.
- [x] The add-device read model now carries a `lanDiscoverySourceMatrix`
      snapshot that maps all 20 LAN plan workpacks and concrete discovery
      sources into typed implemented, partial, manual-required, and
      not-implemented statuses.
- [x] Activity/Network diagnostics render source-matrix rows for LAN workpacks,
      implemented source proof, weak-source fencing, and matrix generation time
      from the service-backed read model.
- [ ] Event types include interface changed, scan started, scan finished,
      evidence found, device found, device updated, online, offline, agent
      discovered, agent confirmed, and unknown detected.
- [ ] Events include event id, timestamp, session/scan id where relevant, and
      affected device id for device events.
- [ ] Read models are derived from canonical registry/evidence state.
- [ ] Portal can replay snapshot plus events without duplicate cards.
- [ ] Empty, unavailable, degraded, stale, offline, agent-offline, and
      manual-required states are explicit.
- [x] Activity / Network diagnostics now show scan/evidence first-seen and
      last-seen timing, evidence expiry, signed adapter proof state, and
      policy-target history.
- [x] Visual snapshot proof exists for the current service-backed Devices/LAN,
      Activity/Network diagnostics, and Network policy target surfaces under
      `output/playwright/lan-source-matrix-plan-completion/`.
- [ ] Replayable scan sessions, full pairing/heartbeat event history, richer
      network-flow evidence remain open.

## Acceptance And Proof

- Contract tests protect the LAN event stream shape.
- Service tests cover event ordering and duplicate event id rejection.
- `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` proves the
  source-matrix read-model field is preserved across TypeScript contracts, Rust
  protocol, Rust service state, and portal render data.
- Playwright contract-fixture UI tests cover empty dashboard, progressive scan,
  evidence panel, assignment, confirmed badge, offline state, and malicious
  hostname rendering. Later real-backend tests cover service-backed routes.
- Visual snapshot proof covers Devices/LAN, Activity/Network, and relevant
  policy target routes so manual review can catch product gaps beyond tests.

## Parallel Ownership Notes

Portal UI work for this LAN plan is part of codex-b ownership. The portal must
not invent LAN state or own merge/registry truth; it renders service-backed
state from the canonical household device/read-model path.
