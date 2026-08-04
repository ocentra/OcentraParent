# V0.8 Enforcement Control Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Use this index to open exactly one assigned workpack. Do not read every file in
`workpacks/`.

| Status  | Workpack                                                                                          |  Size | Boxes               |
| ------- | ------------------------------------------------------------------------------------------------- | ----: | ------------------- |
| checked | [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md)   | 1,310 | 5/5 checked; 0 open |
| checked | [02 Policy Decision Evidence References](workpacks/02-policy-decision-evidence-references.md)     | 1,197 | 5/5 checked; 0 open |
| checked | [03 Adapter Capability Matrix](workpacks/03-adapter-capability-matrix.md)                         | 1,185 | 5/5 checked; 0 open |
| checked | [07 Unmanaged Browser Fallback](workpacks/07-unmanaged-browser-fallback.md)                       | 1,781 | 5/5 checked; 0 open |
| checked | [09 Timer Recovery And Rollback](workpacks/09-timer-recovery-and-rollback.md)                     | 1,667 | 5/5 checked; 0 open |
| checked | [18 Proof Command And Matrix](workpacks/18-proof-command-and-matrix.md)                           | 1,592 | 5/5 checked; 0 open |
| blocked | [04 Owned-Process Time Limit](workpacks/04-owned-process-time-limit.md)                        | 1,096 | 0/5 checked; 5 open; Eventing WP06 generic journal handoff has a durable manifest, but WP04 remains unscheduled/manual-required until WP11 durable-journal proof is present; a recorded blocker does not satisfy scheduling |
| open    | [05 App And Game Session Handoff](workpacks/05-app-game-session-handoff.md)                       | 1,126 | 0/5 checked; 5 open |
| open    | [06 Managed Browser Session Control](workpacks/06-managed-browser-session-control.md)             | 1,127 | 0/5 checked; 5 open |
| open    | [08 Network/Domain Report-Only Boundary](workpacks/08-network-domain-report-only-boundary.md)     | 1,190 | 0/5 checked; 5 open |
| open    | [10 Parent Approval And Override](workpacks/10-parent-approval-override.md)                       | 1,181 | 0/5 checked; 5 open |
| open    | [11 Audit And Journal Events](workpacks/11-audit-journal-events.md)                               | 1,123 | 0/5 checked; 5 open; Eventing WP06 generic proof/handoff has a durable manifest; WP11 still owns the durable enforcement-journal handoff required before WP04 dispatch-ready work |
| open    | [12 Child-Facing Status And Reasons](workpacks/12-child-facing-status-and-reasons.md)             | 1,116 | 0/5 checked; 5 open |
| open    | [13 Service Read Models And API](workpacks/13-service-read-models-and-api.md)                     | 1,074 | 0/5 checked; 5 open |
| open    | [14 Portal Control State Consumption](workpacks/14-portal-control-state-consumption.md)           | 1,093 | 0/5 checked; 5 open |
| open    | [15 Integrity Heartbeat And Permission Loss](workpacks/15-integrity-heartbeat-permission-loss.md) | 1,082 | 0/5 checked; 5 open |
| open    | [16 Tamper/Uninstall Non-Claim Design](workpacks/16-tamper-uninstall-non-claim-design.md)         | 1,125 | 0/5 checked; 5 open |
| open    | [17 Cross-Platform Unavailable States](workpacks/17-cross-platform-unavailable-states.md)         | 1,118 | 0/5 checked; 5 open |
| open    | [19 Playwright And UI Proof](workpacks/19-playwright-ui-proof.md)                                 | 1,145 | 0/5 checked; 5 open |
| open    | [20 Rollout Docs And CI/PR Gate](workpacks/20-rollout-docs-ci-pr-gate.md)                         | 1,139 | 0/5 checked; 5 open |

## Workpack family route

Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected
workpack's owner or handoff boundary is unclear. Do not treat it as permission
to read every sibling plan.

## High-risk open families

- Action authority and adapter execution: Eventing WP06 Journal Replay And Lineage (generic prerequisite has a durable manifest) -> WP11 durable journal handoff -> WP04, then WP05, WP06, WP08. Until the WP11 enforcement-specific handoff is present, WP04 is unscheduled/manual-required.
- Approval, audit, and read-model truth: WP10, WP11, WP13. WP11 is a scheduling prerequisite for WP04, not merely a downstream audit packet.
- Integrity and non-claim boundaries: WP15, WP16, WP17.
- Surface and rollout gate: WP12, WP14, WP19, WP20.

## Closure rule

Checked workpacks prove only their named slices. Broad enforcement readiness
remains blocked until the open workpacks above are either closed with proof or
carried forward explicitly as manual-required or not-ready gaps.
