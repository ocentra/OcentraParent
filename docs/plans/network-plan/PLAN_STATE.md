# Network Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `network-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for child-device network evidence, domain observation, DNS and flow classification, process/app/browser correlation, network-triggered cross-slice evidence cascade, network policy handoff, DNS/firewall/VPN/WFP/NetworkExtension intervention paths, proof artifacts, and parent UI.

## Current ownership interpretation

```text
crates/schema or the owning Rust crate:
  Canonical shared network contracts and route/action/read-model DTOs when shapes cross package, app, crate, or plan boundaries.

schema-domain:
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.

network-core, agent-protocol, and agent-core:
  Rust-owned network domain decisions, canonical protocol contracts, and production runtime/eventing proof surfaces.

ocentra-network-evidence:
  Rust network evidence/proof crate for packet, DNS, domain, flow, classifier, cascade, policy-handoff, platform-gate, adapter, risk, and AI-audit proof helpers.

agent-protocol, agent-core, and agent-service:
  Protocol, runtime-chain, capture, delivery, service read-model, and bridge proof only when the selected workpack names those surfaces.

apps/portal and portal-domain:
  Projection/UI only. Portal renders service-backed network state and does not own network truth.

eventing-plan:
  Reusable local event bus semantics only. Network consumes typed eventing handoffs and must not create a private bus.

Browser, screen, AI, policy, enforcement, LAN, data custody, device-trust, and notification plans:
  Sibling owners for exact URL/page evidence, screen fallback, AI runtime/provider behavior, policy decisions, enforcement actions, LAN delivery, custody, device authority, and notifications.
```

## Current coupling risks

```text
- Canonical shared network contracts live in `crates/agent-protocol`; runtime ownership is `crates/network-core` and `crates/agent-core`.
- Checklist count is not proof completion.
- Shim-cleanup skeleton proof is not workpack completion.
- Schema/unit tests are not live capture proof.
- PCAP or fixture replay is not live capture proof.
- Network evidence is not exact URL, exact video, private message, search text, or private content proof.
- Policy mapping is not enforcement authority.
- Adapter contract or lab proof is not production intervention readiness.
- Control catalog or settings inventory existence is not implementation proof.
```

## Current proof interpretation

```text
output/network-plan-proof/<workpack>/ is the normal deterministic proof root.
docs/proof/network-plan/ currently contains only the slice-01 shim-cleanup proof note.
The slice-01 proof is a bounded parent-domain frontage cleanup proof only.
The broader WP01-WP08 workpacks remain open until their proof roots exist or blockers are explicitly written.
Real platform proof remains required for platform claims unless explicitly marked external-platform constraint or manual-required.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.

## Latest validation slice — 2026-08-09

WP08 control-catalog reference routing now has an executable route-boundary
contract test and durable manifest at
`docs/proof/network-plan/slice-08-control-catalog-routing.md`. The graph state
is `validation`, not `done`: no network runtime, policy, enforcement, portal,
platform, CI, review, or main-merge claim is made by this slice.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-network-snapshot.md](current-network-snapshot.md)

## Audit truth snapshot

Audit refresh on 2026-06-16 for branch `codex/tracking-plan-full-continuation-a` found:

- the canonical network contract source is Rust-owned: `crates/agent-protocol`, with `crates/network-core` and `crates/agent-core` owning domain/runtime behavior;
- real network code exists across `crates/network-core`, `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`, `crates/ocentra-network-evidence`, and `apps/portal`;
- the proof root was missing at audit time, but `docs/proof/network-plan/` and `output/network-plan-proof/01-network-foundation-shim-cleanup/` were restored on 2026-06-17 and now record the bounded parent-domain frontage retirement;
- three bounded WP01 proof documents exist right now: `docs/proof/network-plan/01-network-foundation-shim-cleanup.md` for the retired parent-domain frontage, `docs/proof/network-plan/01-network-foundation-eventing-contract.md` for the typed reusable-eventing handoff, and `docs/proof/network-plan/01-foundation-contracts-and-eventing.md` for the reviewed runtime-contract repair; `PLAN_PROOF_MANIFEST.md`, `PROOF_INDEX.md`, and `PLAN_HEALTH.md` route those receipts while broader plan proof bundles remain missing;
- `implementation-checklist.md` shows 127/128 checked boxes, but that count is not a truthful completion signal while source paths, proof routing, and workpack state are out of sync.

## Current slice checkpoint

- 2026-06-17 `network-foundation-shim-cleanup` is limited to the parent-domain TypeScript ownership boundary plus the initial proof-root restoration; current source truth now carries that slice through the dead-frontage retirement.
- This slice retires `packages/parent-domain/src/network-flow.ts` and `packages/parent-domain/src/network-contracts.ts` because `@ocentra-parent/parent-domain` does not publish `./network-flow` or `./network-contracts`, and no live in-repo consumers were found for those parent-domain paths; it does not widen into Rust, portal, or platform proof.
- The former `@ocentra-parent/parent-domain` `./network-control-catalog` contradiction is already retired as well; canonical `network-flow`, `network-contracts`, and control-catalog ownership remains only in `crates/schema`, the owning Rust crate, or selected network proof surfaces.
- Proof pack for this slice lives at `docs/proof/network-plan/01-network-foundation-shim-cleanup.md` with artifacts under `output/network-plan-proof/01-network-foundation-shim-cleanup/`.
- The current WP01 eventing sub-slice is a direct `NetworkFlowObservedEvent` to reusable `DomainEvent`/`EventEnvelope` contract handoff in `crates/agent-protocol`; its focused round-trip and invalid-device-reference evidence live at `docs/proof/network-plan/01-network-foundation-eventing-contract.md`. This does not close WP01 or establish service, capture, policy, enforcement, or platform proof.

## What is already present in source

- `crates/network-core` owns network domain/runtime decisions; it does not create a second schema truth.
- `crates/agent-protocol` owns real Rust network contracts, status payload shapes, constants, and protocol tests.
- `crates/agent-core` owns real network capture, ActivityStore network rows, runtime chain, queue, replay, and remote-delivery proof logic with tests.
- `crates/agent-service` owns real network payload, digest, runtime-delivery, product-path, remote-delivery, and platform-gate bridge code with tests.
- `crates/ocentra-network-evidence` owns real packet, DNS, domain, classifier, cascade, adapter-gate, performance, and platform-claim proof logic with tests.
- `apps/portal` owns real service-backed network read-model parsing, drawer projection, refresh routing, and e2e proof fixtures.

## Open gaps / truth boundaries

### Real dependency blockers

- Cross-plan rows that depend on browser exact-URL evidence, screen-summary fallback, AI runtime ownership, eventing semantics, LAN/family-hub delivery, or enforcement authority remain dependent on their owning plans.
- The current proof manifest ties the reviewed WP01 contract/runtime slice to its retained proof route; it does not prove broader plan completion, live capture, or platform readiness.

### External platform constraints

- Real macOS and iOS proof is not expected from this Windows host. Those rows remain external-platform constraints until a Mac host produces the required device, entitlement, and runtime artifacts.

### Avoidable local execution gaps on this host

- Windows proof is expected where a row needs it; current gaps are proof-generation and row-tracking gaps, not a host limitation.
- Android tooling is present and an AVD exists, but no device is attached right now and the remembered Samsung Wi-Fi ADB endpoint did not answer during the audit refresh.
- WSL is installed but stopped, and Docker Desktop's binary exists while the Linux engine is currently unavailable; Linux proof through WSL and/or Docker is therefore feasible but not currently active.
- The shim-cleanup proof and the WP01 foundation-contract manifest/receipt exist; all broader plan proof bundles still need to be generated before rows can close honestly.
- Production live packet capture driver support and live raw artifact creation.
- Router/log import implementation proof.
- Local AI model execution or remote provider execution.
- Full policy engine execution and notification provider delivery.
- Live host DNS/WFP/VPN/NetworkExtension/Linux adapter mutation, packet blocking, process termination execution, and host filtering. Windows Firewall has only a bounded reversible TEST-NET lab execution proof; production enforcement and persistent policy-driven firewall rules remain open.
- Device Owner or other authority-enrolled Android proof where a platform claim needs it.
- Parent-facing rule UX and broader risk-budget/performance/platform UI beyond the current service-backed network drawer.
- Production SLO validation, external audit or penetration-test execution, deployment execution, and full support-material authoring.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 128 total, 127 checked, 1 unchecked.
- Current audit rule: do not treat the checkbox count as plan truth while the proof roots are missing and the workpacks remain open.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 8 route workpacks.
- Workpack source: `03-network-implementation-checklist-and-workpacks.md` rows split into focused files under `workpacks/`.
- Workpacks with implementation proof complete: 0.
- Workpacks open: 8.
- Current meaning: the plan is routeable, but live capture, adapter intervention, mobile authority, and production rollout remain unproved unless a selected workpack provides proof.

### Active/open workpacks

- WP01 foundation contracts and eventing.
- WP02 passive capture and parsing.
- WP03 classification and correlation.
- WP04 cross-slice cascade and parent surface.
- WP05 intervention adapter proof gates.
- WP06 analyzer, AI audit, and risk budget.
- WP07 performance, security, and rollout.
- WP08 control catalog reference routing.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless the selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under `docs/proof/network-plan/` or the selected `output/network-plan-proof/<workpack>/` root.
- Current audit truth:
  - `docs/proof/network-plan/` now exists again, but only the slice-01 skeleton is present;
  - do not mark any slice complete until the assigned slice has real committed artifacts beyond the current skeleton or an explicit blocker note;
  - each rebuilt proof file must include commands, pass/fail, negative cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
