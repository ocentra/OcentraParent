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

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-network-snapshot.md](current-network-snapshot.md)

## What is already present / proved

- No concise existing/proved bullet section was detected in the current snapshot.

## Open gaps / missing product runtime

- Production live packet capture driver support and live raw artifact creation.
- Router/log import implementation proof.
- Live broker/family-hub transport, provider delivery, child-device delivery, remote acknowledgement handling, and remote delete/export propagation.
- Local AI model execution or remote provider execution.
- Full policy engine execution and notification provider delivery.
- Live host DNS/WFP/VPN/NetworkExtension/Linux adapter mutation, packet blocking, process termination execution, and host filtering. Windows Firewall has only a bounded reversible TEST-NET lab execution proof; production enforcement and persistent policy-driven firewall rules remain open.
- Physical-device proof beyond the named Android target, Device Owner or other authority-enrolled proof, and any platform adapter execution proof where a platform claim needs it.
- Parent-facing rule UX and broader risk-budget/performance/platform UI beyond the current service-backed network drawer.
- Production SLO validation, external audit or penetration-test execution, deployment execution, and full support-material authoring.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 128 total, 127 checked, 1 unchecked.
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
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/network-plan/.
- Required proof manifest names:
  - docs/proof/network-plan/slice-01-\*.md
  - docs/proof/network-plan/slice-02-\*.md
  - docs/proof/network-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
