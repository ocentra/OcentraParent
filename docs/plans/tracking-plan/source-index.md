# Tracking Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file records source material for `docs/plans/tracking-plan`. It prevents
future workers from re-reading every product document or inventing a second
tracking-control truth.

## Repo Source Inputs

| Source                                                                 | Why it matters                                                                                                                                                   |
| ---------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `docs/features/location-geofence-device-status.md`                     | Feature owner, current status, gap, roadmap anchors, and next AI instructions.                                                                                   |
| `docs/expectations/location-geofence.md`                               | Parent/child outcome, data scope, contract families, validation, and non-goals.                                                                                  |
| `docs/tracking-control-settings-inventory.md`                          | Generated 338-setting inventory, including posture modes, execution modes, capability states, heartbeat, battery, sync, pending upload, and missing-device rows. |
| `docs/device-location-tracking-capability-guide.md`                    | Capability terms, location history, live tracking, geofence, check-in, last known, custody, and platform limits.                                                 |
| `docs/device-location-tracking-schema-proposal.md`                     | Authoring manifest, policy value, effective policy, update protocol, and capability registry guidance.                                                           |
| `docs/expectations/platforms.md`                                       | Platform claim rule and proof requirements for Windows, macOS, Linux, Android, iOS, Web, and parent app boundaries.                                              |
| `docs/expectations/notifications.md`                                   | Notification intent, provider minimization, retry, quiet hours, escalation, and audit requirements.                                                              |
| `docs/expectations/policy.md`                                          | Parent policy owns action authority; portal authors rules but child-device agents validate/evaluate locally.                                                     |
| `docs/expectations/ai.md`                                              | AI is evidence, not authority; remote AI disabled by default; custody and evidence refs are mandatory.                                                           |
| `docs/expectations/data-custody.md`                                    | Local/LAN-first storage, no default Ocentra-hosted child activity store, and explicit export/delete boundaries.                                                  |
| `docs/features/reports-notifications-sync.md`                          | Broad reports, notifications, and sync ownership. Tracking emits location alert intents but does not own provider delivery globally.                             |
| `docs/features/local-ai-safety-evaluator.md`                           | General local AI runtime/provider status ownership. Tracking owns location-specific AI contracts only.                                                           |
| `docs/features/parent-assistant-actions.md`                            | Assistant can explain or draft tracking policy, but must not bypass typed preview, parent confirmation, or child-agent validation.                               |
| `docs/plans/eventing-plan/03-event-taxonomy-and-parent-integration.md` | Shared event taxonomy and Parent runtime boundary. Tracking must add first-class consumer event families instead of creating a private bus.                      |
| `docs/plans/eventing-plan/05-implementation-workpacks.md`              | Reusable `ocentra-eventing` implementation and consumer-proof sequencing. Tracking event work consumes this crate/plan after protocol/domain contracts exist.    |
| `docs/plans/tracking-plan/event-driven-runtime-test-matrix.md`         | Real test matrix for tracking/eventing implementation. Future proof artifacts must cite real source files, real tests, commands, and observed chains.            |

## Product Source Docs

Use the focused product-doc path before implementation:

- feature owner: `docs/features/location-geofence-device-status.md`;
- primary expectation: `docs/expectations/location-geofence.md`;
- supporting expectations: `docs/expectations/platforms.md`,
  `docs/expectations/notifications.md`, `docs/expectations/policy.md`,
  `docs/expectations/ai.md`, and `docs/expectations/data-custody.md`;
- status ledger: `docs/product-capability-checklist.md`;
- milestone context only when status/order changes:
  `docs/product-roadmap.md`.

## Feature Routing

Tracking owns location evidence, geofence decisions, expected-place checks,
device-status evidence, location-specific AI inputs/results, and
tracking-specific parent/child UI. It does not own general notification
delivery, general AI provider runtime, browser telemetry, app/game telemetry,
LAN discovery, or the shared evidence-store feature except through explicit
evidence refs.

Tracking must reuse adjacent runtime infrastructure instead of cloning it:
generic eventing/journal/replay from `crates/ocentra-eventing`, shared evidence
refs and query-store mechanics, LAN/device presence as hint-only inputs,
network/browser/app-game evidence through stored refs, provider status
boundaries, and AI request/result boundary contracts. Tracking-specific code
should only add location/geofence/expected-place/nearby-place meaning,
tracking-side state transitions, and policy-evidence preparation.

Tracking also owns the consumer-layer event contracts for location/geofence
runtime chains: `tracking.*`, `location.*`, `geofence.*`, `expected_place.*`,
`nearby_place.*`, `tracking.live_mode.*`, tracking notification intent/status,
and tracking escalation state. The reusable eventing crate remains generic and
must not depend on tracking product types.

## TypeScript Ownership

TypeScript is a portal/protocol/read-model contract mirror for tracking, not
the runtime decision engine. Current buckets:

- `packages/activity-domain` owns schema-facing activity/location evidence,
  geofence, retention, local place, and tracking read-model contracts used by
  TypeScript consumers.
- `packages/agent-protocol-domain` for portal/agent WebSocket command and
  event contract mirrors.
- `packages/parent-domain` keeps parent policy/product/readiness contracts and
  proof accounting. It must not become the tracking runtime brain.
- `packages/endpoint-domain`, `packages/portal-domain`, and
  `packages/text-domain` for routes, DOM ids, command ids, and display text
  tokens used by portal surfaces.
- `apps/portal` renders service-backed tracking read models and sends typed
  parent intents only.
- `scripts/test/tracking-*.mjs`, hosted proof helpers, artifact inventories,
  handoff proofs, blocker proofs, and closure proofs are proof/accounting
  harnesses. Do not move or expand them as runtime code.

## Rust Ownership

Rust-facing protocol shapes belong in `crates/agent-protocol` only after the
Rust-crossing contract is explicit and test-backed. Runtime tracking behavior
belongs in Rust before portal proof or TypeScript helper expansion.

- `crates/agent-core` owns tracking state helpers, local durable state,
  ActivityStore tracking projections, and platform-neutral tracking behavior
  that should not live in the service shell.
- `crates/agent-protocol` owns Rust tracking structs, constants, command names,
  event names, field names, and state labels.
- `crates/agent-service` owns WebSocket/API transport and event response
  construction only.
- `crates/ocentra-eventing` stays generic; tracking consumes it after
  tracking-specific protocol payloads exist.
- Shared event/journal/replay/provider/status code must stay shared across
  tracking, LAN, network, browser, app/game, notification, and AI lanes. Do not
  create tracking-local copies of common runtime mechanics.
- New Rust tracking tests should live in crate-level `tests/` folders when
  testing public crate APIs.

Runtime service behavior must keep Rust string constants in protocol crates and
must not infer precise location from LAN, IP, or pairing metadata.

## Portal Ownership

Portal work may render tracking surfaces only after the contract/source state
is explicit. Parent UI must show source, accuracy, freshness, custody,
retention, permission/capability status, and stale/offline state instead of
presenting weak evidence as live location.

## Proof Scripts

Future implementation work should route proof through focused commands first
and root-gate only when the workpack is PR-ready:

- TypeScript contract/parser tests for domain packages;
- Rust protocol conversion tests after protocol mirroring;
- service/WebSocket smoke for real local transport;
- Playwright screenshots for parent/child UI states;
- manual Android/iOS/desktop proof scripts for platform claims;
- retention/delete/export proof commands for custody claims.

## Current Test Files

Focused tracking proof scripts now exist under `scripts/test/` and write
evidence under `output/tracking-plan-proof/<workpack-id>/` plus
`test-results/<proof-mode>/`. The source/gap-map reconciliation gate is
`node scripts/test/tracking-source-reconciliation-gap-map-proof.mjs`; the
product-readiness closure gate is
`node scripts/test/tracking-product-readiness-closure-proof.mjs`. Runtime,
platform, hosted UI, provider, authority, production, and manual-required proof
scripts stay scoped to their owning workpacks and proof tiers.

## Source Truth Rule

If a future worker finds conflict between this plan and product docs, the
current feature doc, expectation docs, capability checklist, and roadmap status
win. Update this folder after the source truth changes; do not use this folder
to override product docs.

## Adjacent Plan Boundaries

- `docs/plans/browser-plan/README.md` owns managed browser URL/tab evidence.
  Tracking may use managed URL as schedule or risk context only through stored
  evidence refs.
- `docs/plans/app-plan/README.md` owns native app/game evidence. Tracking may
  use app/game foreground context only through stored evidence refs.
- `docs/plans/lan-plan` owns LAN pairing and device discovery. LAN presence is
  a hint only, not proof of precise child location.
- `docs/features/evidence-store-query.md` owns the shared journal/query-store
  model. Tracking consumes and extends the evidence store; it does not replace
  it.

## Pasted Draft Coverage

Two pasted guides were used:

- `C:\Users\sujan\.codex\attachments\f6b10f30-7802-442a-8199-cd6dbe7b9bcb\pasted-text.txt`
  supplied the full first draft, including product rules, contracts, modes,
  retention, UI, 32-workpack split, tests, proof pack, and final quality bar.
- `C:\Users\sujan\.codex\attachments\da33059b-0d1f-4432-a963-7cea423b32c0\pasted-text.txt`
  supplied the repo-plan-style correction, including browser-plan style,
  mermaid flow, 30-workpack base shape, platform extension checklists,
  implementation checklist gates, test folder structure, and worker
  instructions.

The ChatGPT share URL provided by the user was checked, but only a login shell
and title were visible in this environment. The second pasted attachment is
treated as the accessible GPT guide.

## External Technical Inputs

Workers must re-verify official platform docs before implementation or product
claims. Planning inputs include:

- Android geofencing and background location;
- Android fused/current/last-known location;
- iOS Core Location, region monitoring, background location;
- Apple device management / Lost Mode / supervised devices where applicable;
- Places / POI providers such as Google Places, Apple MapKit, OpenStreetMap;
- notification channels such as local push, parent app, SMS, email, and
  secondary guardian later;
- privacy/legal requirements for child location data, retention, consent,
  deletion, and audit.

## Claim Boundary

This folder does not replace source docs. It turns them into implementation and
proof workpacks. WP01/WP02 source and gap-map proof must continue to cite the
current feature doc, expectation docs, product checklist row, tracking settings
inventory, current snapshot, pasted-content audit, and product-readiness closure
proof instead of inventing another status ledger.

Do not:

- treat pasted TypeScript-like types as runtime code;
- infer precise location from LAN/IP/pairing/network metadata;
- make AI household authority;
- store child location history in Ocentra-hosted systems by default;
- claim Android/iOS background behavior without real device proof;
- use nearby POI data as exact child location;
- make notification providers a child-location data store.
