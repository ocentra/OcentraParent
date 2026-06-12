# V0.8 Enforcement Control Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `v0-8-enforcement-control-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for V0.8 enforcement, product-control action states, adapter proof, integrity state, and parent-visible control readiness.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- No `current-*.md` snapshot exists; use README/implementation indexes until one is added.

## What is already present / proved

- No concise existing/proved bullet section was detected in the current snapshot.

## Open gaps / missing product runtime

- No concise missing/gaps bullet section was detected in the current snapshot.

## Checklist summary

- No `implementation-checklist.md` exists in this plan; use the 20-step/test-blueprint files listed in `DOC_INDEX.md` and `ARCHIVE_INDEX.md` only when assigned.
- Checkbox rows detected: 0 total, 0 checked, 0 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 20.
- Workpacks with open checkboxes: 18.
- Workpacks with all detected boxes checked: 2.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md) - 0/5 checked, 5 open.
- [02 Policy Decision Evidence References](workpacks/02-policy-decision-evidence-references.md) - 0/5 checked, 5 open.
- [03 Adapter Capability Matrix](workpacks/03-adapter-capability-matrix.md) - 0/5 checked, 5 open.
- [04 Owned-Process Time Limit](workpacks/04-owned-process-time-limit.md) - 0/5 checked, 5 open.
- [05 App And Game Session Handoff](workpacks/05-app-game-session-handoff.md) - 0/5 checked, 5 open.
- [06 Managed Browser Session Control](workpacks/06-managed-browser-session-control.md) - 0/5 checked, 5 open.
- [08 Network/Domain Report-Only Boundary](workpacks/08-network-domain-report-only-boundary.md) - 0/5 checked, 5 open.
- [10 Parent Approval And Override](workpacks/10-parent-approval-override.md) - 0/5 checked, 5 open.
- [11 Audit And Journal Events](workpacks/11-audit-journal-events.md) - 0/5 checked, 5 open.
- [12 Child-Facing Status And Reasons](workpacks/12-child-facing-status-and-reasons.md) - 0/5 checked, 5 open.
- [13 Service Read Models And API](workpacks/13-service-read-models-and-api.md) - 0/5 checked, 5 open.
- [14 Portal Control State Consumption](workpacks/14-portal-control-state-consumption.md) - 0/5 checked, 5 open.
- [15 Integrity Heartbeat And Permission Loss](workpacks/15-integrity-heartbeat-permission-loss.md) - 0/5 checked, 5 open.
- [16 Tamper/Uninstall Non-Claim Design](workpacks/16-tamper-uninstall-non-claim-design.md) - 0/5 checked, 5 open.
- [17 Cross-Platform Unavailable States](workpacks/17-cross-platform-unavailable-states.md) - 0/5 checked, 5 open.
- [18 Proof Command And Matrix](workpacks/18-proof-command-and-matrix.md) - 0/5 checked, 5 open.
- [19 Playwright And UI Proof](workpacks/19-playwright-ui-proof.md) - 0/5 checked, 5 open.
- [20 Rollout Docs And CI/PR Gate](workpacks/20-rollout-docs-ci-pr-gate.md) - 0/5 checked, 5 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full 20-step/test-blueprint files unless `DOC_INDEX.md` or the hub assignment names them.
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
  - a proof manifest under docs/proof/v0-8-enforcement-control-plan/.
- Required proof manifest names:
  - docs/proof/v0-8-enforcement-control-plan/slice-01-\*.md
  - docs/proof/v0-8-enforcement-control-plan/slice-02-\*.md
  - docs/proof/v0-8-enforcement-control-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
