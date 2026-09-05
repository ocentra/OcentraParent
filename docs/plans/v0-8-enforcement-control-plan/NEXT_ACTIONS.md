# V0.8 Enforcement Control Plan Next Actions

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file is the short resume list for the next worker. It is derived from open
workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Read `WORKPACK_FAMILIES.md` only if the selected workpack's owner or
   handoff boundary is unclear.
3. Pick only the assigned workpack from the list below.
4. Open that workpack and exact checklist rows only.
5. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are
   updated and validation is listed.

## Route hygiene status

- Ownership doctrine, workpack-family routing, structured proof expectations,
  and proof-index cleanup are now in place.
- Product/runtime closure is still blocked by the open workpacks below.
- PR #606 is closed without merge as an unsafe/no-op policy slice. It is not a
  shortcut around WP04's trusted-dispatch/journal authority gap.

## Highest-open workpacks by dependency

### Consume the evidenced generic Eventing prerequisite without overclaiming it

- Eventing owner packet: `eventing-plan/workpacks/06-journal-replay-and-lineage.md`. Its generic replay/idempotency/journal handoff is now retained as a hand-authored durable manifest under `docs/proof/eventing-plan/`; raw/generated output remains ignored by GEN-1.2. Enforcement does not implement or promote that generic Eventing work into action authority.

### Establish the durable-journal handoff before execution authority

- [11 Audit And Journal Events](workpacks/11-audit-journal-events.md): 5 open of 5 boxes; completed-command retry recovery and exact final-report persistence are focused-green and fail closed without adapter reexecution, but approval/denial/expiry/override coverage plus the enforcement-specific durable query/proof handoff remain open before WP04 can be scheduled for dispatch-ready work.

### Close execution authority first

- [04 Owned-Process Time Limit](workpacks/04-owned-process-time-limit.md): 5 open of 5 boxes; the reachable raw PID/name service path is now fail-closed/manual-required. WP04 remains unscheduled until WP11's enforcement-specific durable-journal handoff and a canonical persisted grant/binding/trusted-issuer composition route the authenticated executor; Eventing WP06's generic prerequisite alone does not satisfy dispatch-ready proof.
- [05 App And Game Session Handoff](workpacks/05-app-game-session-handoff.md): 5 open of 5 boxes.
- [06 Managed Browser Session Control](workpacks/06-managed-browser-session-control.md): 5 open of 5 boxes.
- [08 Network/Domain Report-Only Boundary](workpacks/08-network-domain-report-only-boundary.md): 5 open of 5 boxes.

### Then close approval, audit, and read-model truth

- [10 Parent Approval And Override](workpacks/10-parent-approval-override.md): 5 open of 5 boxes.
- [13 Service Read Models And API](workpacks/13-service-read-models-and-api.md): 5 open of 5 boxes.

### Keep integrity and non-claim state explicit

- [15 Integrity Heartbeat And Permission Loss](workpacks/15-integrity-heartbeat-permission-loss.md): 5 open of 5 boxes.
- [16 Tamper/Uninstall Non-Claim Design](workpacks/16-tamper-uninstall-non-claim-design.md): 5 open of 5 boxes.

### Use the rollout gate only after the slices above are honest

- [20 Rollout Docs And CI/PR Gate](workpacks/20-rollout-docs-ci-pr-gate.md): 5 open of 5 boxes.

### Still open and dependent on the slices above

- [12 Child-Facing Status And Reasons](workpacks/12-child-facing-status-and-reasons.md): 5 open of 5 boxes.
- [14 Portal Control State Consumption](workpacks/14-portal-control-state-consumption.md): 5 open of 5 boxes.
- [17 Cross-Platform Unavailable States](workpacks/17-cross-platform-unavailable-states.md): 5 open of 5 boxes.
- [19 Playwright And UI Proof](workpacks/19-playwright-ui-proof.md): 5 open of 5 boxes.

## Recently closed proof slice

- [02 Policy Decision Evidence References](workpacks/02-policy-decision-evidence-references.md): closed by `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs` after moving the proof harness back to the actual `enforcement-domain` owner path and adding stale-policy-version, missing/malformed decision-reference, ask-parent dry-run, and missing-source rejection coverage with artifacts under `test-results/v0-8-enforcement-policy-dispatch-proof/` and `output/v0-8-enforcement-control-plan-proof/02-policy-decision-evidence-references/`.
- [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md): closed by `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`, `cargo test -p ocentra-parent-agent-protocol enforcement`, and the focused architecture gate with proof artifacts under `output/v0-8-enforcement-control-plan-proof/01-contract-boundary-and-effect-schemas/`.
- [03 Adapter Capability Matrix](workpacks/03-adapter-capability-matrix.md): closed by `node scripts/test/v0-8-supported-adapter-runtime-proof.mjs`, `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`, and `node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs` with proof artifacts under `test-results/v0-8-supported-adapter-runtime-proof/`, `test-results/v0-8-cross-platform-enforcement-capability-proof/`, `test-results/v0-8-broad-os-adapter-runtime-proof/`, and `output/v0-8-enforcement-control-plan-proof/03-adapter-capability-matrix/`.
- [07 Unmanaged Browser Fallback](workpacks/07-unmanaged-browser-fallback.md): closed by `npm run test --workspace @ocentra-parent/enforcement-domain -- v0-8-browser-enforcement-timer-recovery-proof` plus `node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs` with proof artifacts under `test-results/windows-managed-unmanaged-browser-enforcement-proof/` and `output/v0-8-enforcement-control-plan-proof/07-unmanaged-browser-fallback/`.
- [09 Timer Recovery And Rollback](workpacks/09-timer-recovery-and-rollback.md): closed by `node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs` plus focused Rust timer tests with proof artifacts under `test-results/v0-8-enforcement-timer-recovery-mvp/` and `output/v0-8-enforcement-control-plan-proof/09-timer-recovery-and-rollback/`.
- [18 Proof Command And Matrix](workpacks/18-proof-command-and-matrix.md): closed by `node scripts/test/v0-8-enforcement-control-plan-proof.mjs` with proof artifacts under `test-results/v0-8-enforcement-control-plan-proof/` and `output/v0-8-enforcement-control-plan-proof/18-proof-command-and-matrix/`.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact
remaining rows. Do not create a tiny PR that only updates one proof note while
leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s),
proof reference(s), and feature/product docs if product status changed.

Do not use route-doc cleanup, portal rendering, evidence capture, AI output, or
a focused contract pass to claim full enforcement readiness.

## Actioned completion tracker

- [ ] Re-check this plan route from AGENTS/PLAN_STATE and confirm the assigned workpack path.
- [ ] Re-check `WORKPACK_FAMILIES.md` when owner or handoff boundaries are unclear.
- [ ] Update one assigned workpack and matching checklist/proof rows before reporting progress.
- [ ] Record failure conditions, skipped checks, no-claim boundaries, and evidence paths in the selected workpack and proof artifacts.
