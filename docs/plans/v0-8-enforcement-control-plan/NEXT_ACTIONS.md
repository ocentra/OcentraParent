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

This file is the short resume list for the next worker. It is derived from open workpack/checklist status and does not replace the assigned workpack.

## How to use

1. Confirm the hub assignment and lane.
2. Pick only the assigned workpack from the list below.
3. Open that workpack and exact checklist rows only.
4. Do not claim `DONE` unless the assigned workpack's acceptance/proof rows are updated and validation is listed.

## Highest-open workpacks by unchecked boxes

- [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md): 5 open of 5 boxes.
- [02 Policy Decision Evidence References](workpacks/02-policy-decision-evidence-references.md): 5 open of 5 boxes.
- [03 Adapter Capability Matrix](workpacks/03-adapter-capability-matrix.md): 5 open of 5 boxes.
- [04 Owned-Process Time Limit](workpacks/04-owned-process-time-limit.md): 5 open of 5 boxes.
- [05 App And Game Session Handoff](workpacks/05-app-game-session-handoff.md): 5 open of 5 boxes.
- [06 Managed Browser Session Control](workpacks/06-managed-browser-session-control.md): 5 open of 5 boxes.
- [08 Network/Domain Report-Only Boundary](workpacks/08-network-domain-report-only-boundary.md): 5 open of 5 boxes.
- [10 Parent Approval And Override](workpacks/10-parent-approval-override.md): 5 open of 5 boxes.
- [11 Audit And Journal Events](workpacks/11-audit-journal-events.md): 5 open of 5 boxes.
- [12 Child-Facing Status And Reasons](workpacks/12-child-facing-status-and-reasons.md): 5 open of 5 boxes.
- [13 Service Read Models And API](workpacks/13-service-read-models-and-api.md): 5 open of 5 boxes.
- [14 Portal Control State Consumption](workpacks/14-portal-control-state-consumption.md): 5 open of 5 boxes.
- [15 Integrity Heartbeat And Permission Loss](workpacks/15-integrity-heartbeat-permission-loss.md): 5 open of 5 boxes.
- [16 Tamper/Uninstall Non-Claim Design](workpacks/16-tamper-uninstall-non-claim-design.md): 5 open of 5 boxes.
- [17 Cross-Platform Unavailable States](workpacks/17-cross-platform-unavailable-states.md): 5 open of 5 boxes.
- [18 Proof Command And Matrix](workpacks/18-proof-command-and-matrix.md): 5 open of 5 boxes.
- [19 Playwright And UI Proof](workpacks/19-playwright-ui-proof.md): 5 open of 5 boxes.
- [20 Rollout Docs And CI/PR Gate](workpacks/20-rollout-docs-ci-pr-gate.md): 5 open of 5 boxes.

## PR readiness guard

A PR-ready slice should close a named workpack or clearly explain the exact remaining rows. Do not create a tiny PR that only updates one proof note while leaving the assigned workpack/checklist state stale.

Before reporting `DONE` or `PR_READY`, update the workpack, checklist row(s), proof reference(s), and feature/product docs if product status changed.
