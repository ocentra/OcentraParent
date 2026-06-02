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

## Where We Want To Be

LAN events and read models expose scan state, discovered devices, trusted
devices, route state, controller/observer authority, child-agent presence,
manual proof requirements, stale/offline state, evidence details, and
Activity/Network diagnostics that make service behavior visually inspectable.

## Requirement Checklist

- [ ] Event types include interface changed, scan started, scan finished,
      evidence found, device found, device updated, online, offline, agent
      discovered, agent confirmed, and unknown detected.
- [ ] Events include event id, timestamp, session/scan id where relevant, and
      affected device id for device events.
- [ ] Read models are derived from canonical registry/evidence state.
- [ ] Portal can replay snapshot plus events without duplicate cards.
- [ ] Empty, unavailable, degraded, stale, offline, agent-offline, and
      manual-required states are explicit.
- [ ] Activity / Network diagnostics show scan sessions, evidence sources,
      merge decisions, pairing/heartbeat/revocation events, route rejections,
      network-flow evidence, weak-proof behavior, and policy target binding.

## Acceptance And Proof

- Contract tests protect the LAN event stream shape.
- Service tests cover event ordering and duplicate event id rejection.
- Playwright mocked-backend tests cover empty dashboard, progressive scan,
  evidence panel, assignment, confirmed badge, offline state, and malicious
  hostname rendering. Later real-backend tests cover service-backed routes.
- Visual snapshot proof covers Devices/LAN, Activity/Network, and relevant
  policy target routes so manual review can catch product gaps beyond tests.

## Parallel Ownership Notes

Portal UI work for this LAN plan is part of codex-b ownership. The portal must
not invent LAN state or own merge/registry truth; it renders service-backed
state from the canonical household device/read-model path.
