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

Generated from the existing `v0-8-enforcement-control-plan` docs. This is the
default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for the V0.8 enforcement
control plane: the policy-to-action execution boundary for product-control
action states, adapter proof, integrity state, rollback/recovery, audit, and
parent-visible readiness.

## Canonical ownership doctrine

- `schema-domain` owns canonical shared enforcement schemas when action,
  capability, audit, reason, or read-model shapes cross package, crate,
  protocol, or plan boundaries.
- `policy-control-plane-plan` owns policy source truth, schedule/budget rules,
  ask-parent/override semantics, and parent authorization before an enforcement
  handoff exists.
- `v0-8-enforcement-control-plan` owns the transition from deterministic policy
  decision refs to adapter capability, execution state, rollback/recovery,
  audit, and parent/child visible control state.
- `enforcement-domain` is a helper, proof, and read-model consumer surface. It
  is not the silent canonical owner of cross-boundary schemas.
- `agent-protocol` and `agent-protocol-domain` own protocol parity and
  transport/read-model seams only.
- `app-game`, `browser`, `network`, `screen`, `tracking`, and AI/evidence plans
  own their source facts and evidence surfaces. `portal` owns rendered
  presentation and typed user intent only.

## Enforcement decision chain

```text
policy decision refs
-> actor / device / household authority
-> target and evidence refs
-> adapter capability and platform state
-> observe-only | dry-run | report-only | manual-required | dispatch-ready | rejected
-> execution result | no-op | mismatch | unavailable
-> rollback / recovery / expiry / override
-> audit / journal
-> parent-visible and child-visible state
```

## Hard non-claims

- AI classification is not enforcement authority.
- Portal intent or button state is not enforcement authority.
- Screen, browser, app/game, network/domain, or tracking evidence is not
  enforcement authority by itself.
- Managed-browser readiness is not exact-URL proof.
- Network visibility is not network blocking proof.
- Heartbeat, stale/offline state, or install visibility is not anti-tamper
  proof.
- Any missing link in the decision chain keeps the state manual-required,
  dry-run, report-only, or rejected.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner or handoff boundaries are
   unclear.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` before DONE/PR_READY.

## Current snapshot source

- No `current-*.md` snapshot exists; use the route/state/workpack docs in this
  folder as current truth and treat preserved READMEs as historical context
  only.

## Closed PR disposition

PR #606 is closed without merge. Its policy receipt slice was unsafe/no-op and
is not runtime or proof evidence for this plan. Do not revive it as an
implementation shortcut: WP04 still needs the trusted-dispatch/journal boundary
that can establish authority before execution state, replay/rollback, and
parent-visible receipt claims are considered.

## What is already present / proved

- WP01 contract boundary and Effect Schema ownership is now backed by
  `packages/enforcement-domain/src/enforcement.ts`,
  `crates/agent-protocol/src/enforcement.rs`,
  `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`,
  `cargo test -p ocentra-parent-agent-protocol enforcement`,
  `output/v0-8-enforcement-control-plan-proof/01-contract-boundary-and-effect-schemas/`,
  and
  `docs/proof/v0-8-enforcement-control-plan/slice-04-contract-boundary-and-effect-schemas.md`.
- WP02 policy decision evidence references is now backed by
  `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`,
  `packages/enforcement-domain/src/enforcement-policy-dispatch.ts`,
  `crates/agent-core/src/enforcement_policy_dispatch.rs`,
  `crates/agent-service/src/enforcement_policy_dispatch_read_model.rs`,
  `test-results/v0-8-enforcement-policy-dispatch-proof/`,
  `output/v0-8-enforcement-control-plan-proof/02-policy-decision-evidence-references/`,
  and
  `docs/proof/v0-8-enforcement-control-plan/slice-06-policy-decision-evidence-references.md`.
- WP03 adapter capability matrix is now backed by
  `node scripts/test/v0-8-supported-adapter-runtime-proof.mjs`,
  `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`,
  `node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs`,
  `output/v0-8-enforcement-control-plan-proof/03-adapter-capability-matrix/`,
  and
  `docs/proof/v0-8-enforcement-control-plan/slice-05-adapter-capability-matrix.md`.
- WP07 unmanaged browser fallback is now backed by
  `npm run test --workspace @ocentra-parent/enforcement-domain -- v0-8-browser-enforcement-timer-recovery-proof`,
  `node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`,
  `output/v0-8-enforcement-control-plan-proof/07-unmanaged-browser-fallback/`,
  and
  `docs/proof/v0-8-enforcement-control-plan/slice-01-unmanaged-browser-fallback.md`.
- WP09 timer recovery and rollback is now backed by
  `node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs`,
  `cargo test -p ocentra-parent-agent-core enforcement_timer_state`,
  `cargo test -p ocentra-parent-agent-service enforcement_timer`,
  `output/v0-8-enforcement-control-plan-proof/09-timer-recovery-and-rollback/`,
  and
  `docs/proof/v0-8-enforcement-control-plan/slice-02-timer-recovery-and-rollback.md`.
- WP18 proof command is now present:
  `node scripts/test/v0-8-enforcement-control-plan-proof.mjs`
- Current WP18 proof artifacts:
  `test-results/v0-8-enforcement-control-plan-proof/proof.json`,
  `output/v0-8-enforcement-control-plan-proof/18-proof-command-and-matrix/`, and
  `docs/proof/v0-8-enforcement-control-plan/slice-03-proof-command-and-matrix.md`.

## Open gaps / missing product runtime

- Remaining gaps are tracked by the still-open workpacks below. WP02 is now
  backed by the focused enforcement-domain contract, adapter, Rust-core,
  Rust-service, and proof-harness validation path rather than the broken
  `parent-domain` indirection.
- Action-authority and adapter-execution gaps remain open in WP04, WP05, WP06,
  and WP08. Eventing WP06 now retains a hand-authored durable manifest for its
  generic journal/replay handoff, but WP04 remains unscheduled/manual-required until WP11 supplies
  enforcement-specific durable-journal proof and trusted dispatch. The closed
  #606 unsafe/no-op slice does not reduce this gap.
- Approval/audit/read-model visibility gaps remain open in WP10, WP11, WP12,
  WP13, and WP14. WP11's Eventing prerequisite is documented by a durable manifest; WP11
  remains open until its own durable audit/journal contract and query proof exist.
- Integrity and anti-claim boundaries remain open in WP15, WP16, and WP17.
- Playwright/UI and rollout gate closure remain open in WP19 and WP20.
- Notification delivery, exact-URL control, network blocking, broad app
  blocking, mobile/platform parity, and anti-tamper hardening remain unproved.

## Checklist summary

- No `implementation-checklist.md` exists in this plan; use the 20-step/test-blueprint files listed in `DOC_INDEX.md` and `ARCHIVE_INDEX.md` only when assigned.
- Checkbox rows detected: 0 total, 0 checked, 0 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 20.
- Workpacks with open checkboxes: 14.
- Workpacks with all detected boxes checked: 6.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [04 Owned-Process Time Limit](workpacks/04-owned-process-time-limit.md) - 0/6 checked, 6 open. WP04-S1 requires the production authenticated parent-runtime delivery/outbox and receipt path; agent-side test seeding or raw-envelope rejection alone is insufficient.
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
- [19 Playwright And UI Proof](workpacks/19-playwright-ui-proof.md) - 0/5 checked, 5 open.
- [20 Rollout Docs And CI/PR Gate](workpacks/20-rollout-docs-ci-pr-gate.md) - 0/5 checked, 5 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full 20-step/test-blueprint files unless `DOC_INDEX.md` or the hub assignment names them.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- Route docs now include explicit ownership doctrine, `WORKPACK_FAMILIES.md`,
  structured enforcement E2E tiers, and a cleaned `PROOF_INDEX.md`.
- This route cleanup is docs-only. It does not change runtime closure status.
- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then
    this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned
    implementation boundary,
  - a proof manifest under docs/proof/v0-8-enforcement-control-plan/.
- Required proof manifest names:
  - docs/proof/v0-8-enforcement-control-plan/slice-01-\*.md
  - docs/proof/v0-8-enforcement-control-plan/slice-02-\*.md
  - docs/proof/v0-8-enforcement-control-plan/slice-03-\*.md
  - docs/proof/v0-8-enforcement-control-plan/slice-04-\*.md
  - docs/proof/v0-8-enforcement-control-plan/slice-05-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and
  rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
