<!-- agent-capsule -->

> Agent Capsule
> Doc: Current Workstream Ownership And Docs Plan
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Current Workstream Ownership And Docs Plan

Date: 2026-06-01

This is the current coordinator plan for turning the product back into broad,
testable workstreams instead of scattered proof-only fragments. It also records
how the loose root docs should be organized before future agents rely on them.

## Operating Model

Primary acts as the coordinator and "E" lane. E owns:

- roadmap/doc hygiene and workstream slicing;
- hub and lane state;
- PR creation, review, CI watching, merge, and post-merge pull/rebase notices;
- small live fixes that are clearly integration or checker work;
- user-facing verification of the running portal, service, ports, and visible
  behavior.

A/B/C/D own broad feature work. Assignments should be end-to-end product
workstreams, not small isolated TODOs. When a worker reports `DONE`, primary
reviews the diff and validation before PR/merge. If a bug is small and
integration-owned, E fixes it. If it changes the product spine, it goes back to
the owning A/B/C/D workstream.

## Source Of Truth

Use these files before any broad product assignment:

- `docs/feature-list.md`
- `docs/product-capability-checklist.md`
- `docs/product-roadmap.md`
- `docs/product-constitution.md`
- `docs/full-platform-portal-ai-execution-plan.md`
- the one or two `docs/features/*.md` files that own the work
- the expectation files linked by those feature docs

Do not treat old checkpoint wording or root inventory files as stronger than
the current feature list, capability checklist, roadmap, or constitution.
If an older execution-plan note conflicts with this file about lane ownership,
especially whether C is user-only, this file is the current coordinator
override until the older note is updated.

## Current Product Reality

The immediate user-visible problem is that the portal has UI work, but the user
cannot yet reliably see real household state across screens. The current goal is
therefore:

1. Discover local network devices and label them by stable identity, not just IP.
2. Merge duplicate physical-device representations into one device row when the
   same host exposes portal, child-agent, parent-controller, observer, or
   AI-provider roles.
3. Show real child-agent/device role state in Devices, Policy, Activity, Network,
   Tracking, AI, and Account surfaces.
4. Let a parent click a device and see a real service-backed detail view:
   hostname, addresses, MAC/vendor when available, OS/platform, CPU, GPU,
   memory, service role badges, route status, pairing/trust state, stale/offline
   state, capability support, and latest activity/policy evidence.
5. Keep routers, phones, TVs, printers, and unsupported devices visible as LAN
   inventory, but do not imply that a Rust child agent can be installed on them.

V0.8 and V0.9 are not done. V0.8 still needs real broad enforcement adapter
proof beyond owned-process and managed-session proof. V0.9 still needs
production household discovery, physical multi-device proof, route-source UI,
and mobile controller/observer behavior. V7 and V8 are downstream until these
earlier product paths are visible and honest.

## Root Docs Organization

The root `docs` folder currently mixes canonical product docs with inventories,
schema proposals, UI notes, and generated research. Keep canonical product docs
at root and move supporting docs into purpose folders in a later mechanical
cleanup PR that also updates links.

### Keep At Root

- `feature-list.md`
- `product-capability-checklist.md`
- `product-roadmap.md`
- `product-constitution.md`
- `feature-expectations.md`
- `competitor-capability-map.md`
- `full-platform-portal-ai-execution-plan.md`

`full-platform-portal-ai-execution-plan.md` stays at root only while it is an
active execution reference. It is subordinate to the constitution, roadmap,
capability checklist, feature list, and this current workstream plan for lane
routing. It also contains older C-lane/user-owned wording that must not block
assigning C broad portal UX work now that the user has handed C back to the
coordinator.

### Move To `docs/inventories/`

- `app-control-settings-inventory.md`
- `browser-control-1057-settings-inventory.md`
- `game-control-settings-inventory.md`
- `network-control-settings-inventory.md`
- `screen-control-settings-inventory.md`
- `tracking-control-settings-inventory.md`

Local candidate:

- `browser-policy-decision-forest-full-1057.md` is currently untracked in the
  primary checkout. Its opening content matches the generated 1057 browser
  settings inventory but uses a different numbering style. Do not commit or move
  it blindly; first decide whether it replaces
  `browser-control-1057-settings-inventory.md`, becomes a distinct decision
  forest artifact under `docs/inventories/`, or is discarded as a generated
  duplicate. No worker should treat this untracked file as current scope until
  that decision is made.

### Move To `docs/catalogs/`

- `browser-control-coverage-matrix.md`
- `browser-policy-questionnaire-forest-v1.md`
- `browser-policy-settings-catalog.md`

### Move To `docs/capability-guides/`

- `app-control-capability-guide.md`
- `device-location-tracking-capability-guide.md`
- `game-control-capability-guide.md`
- `managed-unmanaged-browser.md`
- `network-control-capability-guide.md`
- `screen-evidence-analysis-capability-guide.md`

### Move To `docs/schema-proposals/`

- `app-control-schema-proposal.md`
- `browser-control-schema-proposal.md`
- `device-location-tracking-schema-proposal.md`
- `game-control-schema-proposal.md`
- `network-control-schema-proposal.md`
- `screen-evidence-analysis-schema-proposal.md`

### Move To `docs/ui-plans/`

- `data and AI Ui plan.md`
- `manage UI proof checklist.md`
- `policy Ui fix.md`
- `portal and account Ui fix.md`

The doc cleanup itself is a separate coordination-safe PR because moving these
files requires link updates across roadmap, feature, expectation, and README
files. Until that move lands, workstream assignments should cite the current
paths exactly. That cleanup PR must include `rg` proof for old `docs/*.md`
references, markdown link updates, and any literal `sourceDocument` path rewrites
inside generated inventory content.

## Workstream A: Enforcement And Product Control Spine

Lane: `codex-a`

Current gate:

- PR #211, `codex/v0-8-enforcement-browser-adapter-proof`, is the current A
  integration branch. A should not stack the next branch until PR #211 is merged
  or the primary explicitly asks for a stacked fix.

Read before coding:

- `docs/features/enforcement-integrity-tamper.md`
- `docs/features/browser-web-control.md`
- `docs/features/app-game-control.md`
- `docs/features/network-domain-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/browser-evidence.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/network-flow-evidence.md`
- `docs/expectations/policy.md`
- `docs/roadmaps/roadmap-v0-8-enforcement-adapters.md`
- `docs/product-capability-checklist.md`
- `docs/product-roadmap.md`, V0.8 and Current Next Actions

Ownership:

- Take V0.8 from proof spine to honest product-control wiring.
- Keep capability state granular: owned-process limit, managed browser/session,
  unmanaged browser, broad app blocking, network/domain blocking, tamper,
  permission loss, restart recovery, child-facing explanation, rollback, and
  audit must be separately represented.
- Add or harden domain contracts first, then Rust protocol/service parity, then
  portal-facing read/write command state.
- Keep unsupported/manual-required states explicit. Do not claim router control,
  unmanaged browser control, network blocking, or broad app blocking until there
  is a real adapter path and evidence.
- Feed C and D enough typed state so Policy and Devices can show whether a
  device can observe, warn, time-limit, block, ask-parent, or only report.

Deliverables:

- Domain and Rust contracts for next adapter/control states.
- Real-service proof commands and tests.
- Portal/service read model fields for C/D to consume.
- Feature doc and capability checklist updates for any status movement.
- PR-ready branch with validation and known-gaps report.

Validation expectation:

- Focused TypeScript contract tests.
- Focused Rust protocol/service tests.
- Relevant proof harnesses.
- `npm run lint:schema-boundaries`.
- `cargo fmt --all --check`.
- Worker `DONE` must name exact commands, changed files, commit, pushed state,
  PR URL if opened, updated feature docs/checklist rows, and manual gaps.

## Workstream B: Household Device, LAN, Pairing, And Inventory Spine

Lane: `codex-b`

Read before coding:

- `docs/features/family-setup-device-roles.md`
- `docs/features/child-agent-local-service.md`
- `docs/features/remote-lan-mobile-platforms.md`
- `docs/features/evidence-store-query.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/lan-pairing.md`
- `docs/expectations/platforms.md`
- `docs/expectations/real-evidence-proof.md`
- `docs/expectations/data-custody.md`
- `docs/roadmaps/roadmap-v0-9-lan-pairing-multi-device-local-control.md`
- `docs/product-capability-checklist.md`
- `docs/product-roadmap.md`, V0.9 and Current Next Actions
- `docs/full-platform-portal-ai-execution-plan.md`, Roles Not Separate Products

Ownership:

- Make the household device registry canonical. One physical device should have
  one identity with role badges for parent-controller, parent-observer,
  child-agent, portal, and AI-provider.
- LAN inventory must discover and present all reachable local network devices
  as honestly as the host OS allows: hostname where available, IPs, MAC/vendor
  when available, reachability, source, confidence, stale/offline state, router
  or unsupported classification, and child-agent presence.
- When the Rust child agent is reachable, enrich the same device entry with
  service-backed details: device name, platform, OS, CPU, GPU including NVIDIA
  when available, memory, network interfaces, capabilities, role state, route
  state, and pairing/trust state.
- Keep routers and unsupported devices visible but non-enrollable unless there
  is a real supported agent path.
- Persist enough selected-device/registry state that Devices, Policy, Activity,
  Network, Tracking, and AI screens do not lose the known child-agent devices
  when navigating.

Deliverables:

- Canonical device identity and merge rules.
- LAN discovery/read-model updates and real local proof.
- Child-agent capability enrichment contracts and Rust service fields.
- Route-source and trust/pairing states suitable for C/D.
- Feature doc and capability checklist updates for any status movement.
- PR-ready branch with validation and known-gaps report.

Validation expectation:

- Parent-domain tests for device identity, merge, role, and inventory state.
- Rust service tests for discovery/enrichment/read models.
- Real local LAN proof where available, with manual-required notes for two
  physical devices if the current host cannot prove it alone.
- No duplicate `local-dev-agent` plus IP-only rows for the same physical host.

## Workstream C: Parent Portal Product UX Over Real State

Lane: `codex-c`

Current gate:

- C is no longer user-only. Primary owns coordination for C unless the user
  explicitly takes it back.
- The lane is claimed for `codex/portal-ux-real-household-surfaces`, but the
  live worktree still needs to switch from the old C branch before new work.

Read before coding:

- `docs/features/family-setup-device-roles.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/features/browser-web-control.md`
- `docs/features/app-game-control.md`
- `docs/features/network-domain-control.md`
- `docs/features/local-ai-safety-evaluator.md`
- `docs/features/parent-assistant-actions.md`
- `docs/features/reports-notifications-sync.md`
- `docs/expectations/portal.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/policy.md`
- `docs/expectations/browser-evidence.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/network-flow-evidence.md`
- `docs/expectations/ai.md`
- `docs/expectations/parent-assistant-chat.md`
- `docs/expectations/notifications.md`
- `docs/product-capability-checklist.md`
- root UI notes until moved: `data and AI Ui plan.md`,
  `manage UI proof checklist.md`, `policy Ui fix.md`,
  `portal and account Ui fix.md`

Ownership:

- Make the portal render real service-backed state first. Visual polish follows
  once data is not fake.
- Devices should be the parent-visible household source of truth:
  all LAN devices, child-agent devices, role badges, source labels, route state,
  and a detail panel with real capability and hardware/service info.
- Policy Family and Per Device tabs should use the canonical household device
  registry. Family view should show all family scope; Per Device should make
  only child-agent/control-capable devices selectable.
- Activity, Network, Tracking, Browser, Apps, Games, Screen, AI, Reports, and
  Account should show empty/degraded/unavailable/service-backed states from the
  real adapter, not reset to fake arrays per page.
- Remove confusing route duplication such as clicking Devices and landing on a
  separate pairing concept without clear labels. Pairing should be an action or
  mode inside the Devices/household flow unless the product doc says otherwise.
- Keep source/custody labels visible: local, LAN, paired child-agent, router,
  unsupported, stale, offline, observer-only, controller, manual-required.

Deliverables:

- Real household/device shared UI adapter consumption.
- Devices detail state and badges that match B/D contracts.
- Policy device target selection backed by the same registry.
- Real empty/degraded/loading/error states.
- Portal E2E/smoke coverage for service-backed devices and policy target state.
- Feature doc and capability checklist updates for any status movement.

Validation expectation:

- Portal tests for route/state transitions.
- Focused E2E against the real Rust service where possible.
- Screenshot or route-smoke proof that Devices and Policy use the same device
  registry.

## Workstream D: Runtime, Tauri, Mobile, And Service Transport Spine

Lane: `codex-d`

Read before coding:

- `docs/features/remote-lan-mobile-platforms.md`
- `docs/features/child-agent-local-service.md`
- `docs/features/family-setup-device-roles.md`
- `docs/features/production-distribution-support.md`
- `docs/features/reports-notifications-sync.md`
- `docs/expectations/platforms.md`
- `docs/expectations/platform-deliverables.md`
- `docs/expectations/lan-pairing.md`
- `docs/expectations/cloud.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/sync-export.md`
- `docs/roadmaps/roadmap-v6-mobile-agents.md`
- `docs/roadmaps/roadmap-v8-production-hardening.md`
- `docs/product-roadmap.md`, V6, V8, and Current Next Actions
- `docs/full-platform-portal-ai-execution-plan.md`, Platform Plan

Ownership:

- Ensure Vite, Tauri desktop, and mobile scaffolds all consume the same typed
  local/LAN service state. Vite is a development UI surface, not the product
  backend.
- Own service lifecycle and route-source behavior: fixed dev ports, no random
  blank browser windows, no conflict with Ocentra Games port 3000, stale Ocentra
  Parent process cleanup, service health, LAN origin allowlist, and Tauri
  command proof.
- Make parent desktop packaged proof expose the same household device/role state
  that B and C use.
- Keep Android/iOS/mobile child-agent claims honest as scaffold/manual-required
  until real device or emulator proof exists.
- Prepare runtime surfaces for later optional relay without doing remote
  desktop work now.

Deliverables:

- Shared runtime transport contracts and service adapters for portal/Tauri/mobile.
- Dev launch scripts that open the right URL and do not spawn blank pages.
- Package/Tauri proof updates when service-backed household state changes.
- Route-status and degraded-state fields that C can render.
- Feature doc and capability checklist updates for any status movement.

Validation expectation:

- Portal-to-Rust smoke tests on fixed ports.
- Tauri command/package checks where practical.
- Mobile scaffold tests or explicit manual-required notes.
- Proof that port 3000 is not touched by Ocentra Parent scripts.

## Deferred But Tracked Work

These are not ignored, but they should not distract the current real-device UI
goal:

- V7 billing/subscription/account surface.
- V8 production signing, release channels, support, privacy/legal, updater
  rollback, and public download/account/status site.
- Remote desktop capability. The current priority is LAN, device discovery,
  role/capability visibility, and service-backed parent UI. Remote capability
  fabric remains later optional routing/control work.
- Full social/video/location product surfaces after the core household/device,
  policy, AI, and runtime spines are coherent.

## Coordinator Review Gates

Primary should treat worker `DONE` as review-ready, not merge-ready.

Before PR or merge:

1. Inspect the branch diff against current `main`.
2. Confirm touched paths match the workstream.
3. Confirm focused validation is real and exact.
4. Confirm feature docs and capability checklist rows were updated, or the
   worker explicitly explains why no product-doc update was needed.
5. Confirm no product claim overstates proof.
6. Create or update the PR only after branch push and acceptable local proof.
7. Merge only after green CI and acceptable diff review.

After merge:

1. Pull latest `main`.
2. Tell active lanes to fetch/rebase latest `main`.
3. Update hub/lane state.
4. Keep the next assignment broad and end-to-end unless a small integration bug
   is clearly primary-owned.
