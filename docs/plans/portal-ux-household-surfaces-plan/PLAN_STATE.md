# Portal UX Household Surfaces Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `Portal UX Household Surfaces Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `portal-ux-household-surfaces-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for the C lane: coherent parent-facing household UX across setup, devices, policy, activity, LAN state, assistant/report surfaces, degraded states, and proof.

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
- Workpacks with open checkboxes: 15.
- Workpacks with all detected boxes checked: 5.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [05 Policy Authoring Control Center](workpacks/05-policy-authoring-control-center.md) - 0/5 checked, 5 open.
- [06 Schedules, Time Budgets, And Templates](workpacks/06-schedules-time-budgets-and-templates.md) - 0/5 checked, 5 open.
- [07 Parent Requests And Approvals](workpacks/07-parent-requests-and-approvals.md) - 0/5 checked, 5 open.
- [08 Activity Evidence And Diagnostics](workpacks/08-activity-evidence-and-diagnostics.md) - 0/5 checked, 5 open.
- [09 Browser, App, And Network Surfaces](workpacks/09-browser-app-and-network-surfaces.md) - 0/5 checked, 5 open.
- [11 Assistant Action Preview Flow](workpacks/11-assistant-action-preview-flow.md) - 0/5 checked, 5 open.
- [12 Reports, Notifications, And Custody](workpacks/12-reports-notifications-and-custody.md) - 0/5 checked, 5 open.
- [13 Degraded, Empty, Stale, And Error States](workpacks/13-degraded-empty-stale-and-error-states.md) - 0/5 checked, 5 open.
- [14 Audit History And Copy/Debug](workpacks/14-audit-history-and-copy-debug.md) - 0/5 checked, 5 open.
- [15 Accessibility, Responsive, And Keyboard UX](workpacks/15-accessibility-responsive-keyboard-ux.md) - 0/5 checked, 5 open.
- [16 No-Fake-Data Contract Adapter](workpacks/16-no-fake-data-contract-adapter.md) - 0/5 checked, 5 open.
- [17 Playwright Screenshot Proof](workpacks/17-playwright-screenshot-proof.md) - 0/5 checked, 5 open.
- [18 Parent Mobile Shell Readiness](workpacks/18-parent-mobile-shell-readiness.md) - 1/6 checked, 5 open.
- [19 Product Docs And Checklist Sync](workpacks/19-product-docs-and-checklist-sync.md) - 0/5 checked, 5 open.
- [20 Manual User Review Gate](workpacks/20-manual-user-review-gate.md) - 0/5 checked, 5 open.

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
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/portal-ux-household-surfaces-plan/.
- Required proof manifest names:
  - docs/proof/portal-ux-household-surfaces-plan/slice-01-\*.md
  - docs/proof/portal-ux-household-surfaces-plan/slice-02-\*.md
  - docs/proof/portal-ux-household-surfaces-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
