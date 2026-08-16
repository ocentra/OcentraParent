# Browser Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Use this index to open exactly one assigned workpack. Do not read every file in `workpacks/`.

Audit note: as of the 2026-06-16 browser-plan audit, the numbered workpack
files still contain open checklist items, and the expected
`output/browser-plan-proof/<workpack-file-stem>/` roots are absent in this
checkout. Treat this file as a router, not completion truth.

Production-code routing note (2026-08-16): no new reachable implementation
slice is authorized after the existing WP18/WP13 bridges. WP11 requires a real
focus/activation provider; WP21 requires extension/native-host packaging and
registration; WP20 requires Windows AppLocker/WDAC authority; and WP22 requires
a runtime performance-health producer. These workpacks remain open and
code-blocked; validation, proof, and platform gaps must not be promoted to
production implementation. The graph report's all-`planned` topology is stale
and is not completion evidence.

| Status  | Workpack                                                                                            |   Size | Boxes                 |
| ------- | --------------------------------------------------------------------------------------------------- | -----: | --------------------- |
| open    | [01 Contract Boundary And Effect Schemas](workpacks/01-contract-boundary-and-effect-schemas.md)     |  5,608 | 0/16 checked; 16 open |
| open    | [02 Source Index And Doc Reconciliation](workpacks/02-source-index-and-doc-reconciliation.md)       |  4,364 | 0/16 checked; 16 open |
| open    | [03 Browser Inventory Model](workpacks/03-browser-inventory-model.md)                               |  5,926 | 0/16 checked; 16 open |
| open    | [04 Windows Browser Inventory Adapter](workpacks/04-windows-browser-inventory-adapter.md)           | 14,183 | 0/16 checked; 16 open |
| open    | [05 Cross-Platform Inventory Matrix](workpacks/05-cross-platform-inventory-matrix.md)               | 21,493 | 0/16 checked; 16 open |
| open    | [06 Managed Profile Store](workpacks/06-managed-profile-store.md)                                   |  5,199 | 0/16 checked; 16 open |
| open    | [07 Managed Chromium Launcher](workpacks/07-managed-chromium-launcher.md)                           |  5,377 | 0/16 checked; 16 open |
| open    | [08 Bridge Custody And Security](workpacks/08-bridge-custody-and-security.md)                       |  3,712 | 0/16 checked; 16 open |
| open    | [09 CDP Version And Target Adapter](workpacks/09-cdp-version-and-target-adapter.md)                 |  4,576 | 0/16 checked; 16 open |
| open    | [10 Tab Evidence Mapper](workpacks/10-tab-evidence-mapper.md)                                       |  4,678 | 0/16 checked; 16 open |
| open    | [11 Active-Tab Proof Model](workpacks/11-active-tab-proof-model.md)                                 |  3,860 | 0/16 checked; 16 open |
| open    | [12 Journal And SQLite Browser Ingest](workpacks/12-journal-and-sqlite-browser-ingest.md)           |  3,851 | 0/16 checked; 16 open |
| open    | [13 Browser Read Models And Service Events](workpacks/13-browser-read-models-and-service-events.md) | 82,422 | 0/16 checked; 16 open |
| open    | [14 Portal Browser Status Surfaces](workpacks/14-portal-browser-status-surfaces.md)                 |  6,579 | 0/16 checked; 16 open |
| open    | [15 Browser Policy Authoring Manifest](workpacks/15-browser-policy-authoring-manifest.md)           |  5,076 | 0/16 checked; 16 open |
| open    | [16 Policy Target Compiler](workpacks/16-policy-target-compiler.md)                                 |  6,644 | 0/16 checked; 16 open |
| open    | [17 Managed Intervention And Block Page](workpacks/17-managed-intervention-and-block-page.md)       |  6,431 | 0/16 checked; 16 open |
| open    | [18 Unmanaged Browser Detection](workpacks/18-unmanaged-browser-detection.md)                       |  4,075 | 0/21 checked; 21 open |
| open    | [19 Unmanaged Fallback UX And Actions](workpacks/19-unmanaged-fallback-ux-and-actions.md)           |  4,253 | 0/16 checked; 16 open |
| open    | [20 Windows AppLocker And App Control Proof](workpacks/20-windows-applocker-app-control-proof.md)   |  4,625 | 0/16 checked; 16 open |
| open    | [21 Extension And Native Host Boundary](workpacks/21-extension-and-native-host-boundary.md)         |  5,112 | 0/16 checked; 16 open |
| open    | [22 Performance And Service Health](workpacks/22-performance-and-service-health.md)                 |  5,363 | 0/16 checked; 16 open |
| open    | [23 E2E And Manual Proof Artifacts](workpacks/23-e2e-and-manual-proof-artifacts.md)                 |  6,229 | 0/16 checked; 16 open |
| open    | [24 Rollout, Checklist, And PR Gate](workpacks/24-rollout-checklist-and-pr-gate.md)                 |  5,502 | 0/16 checked; 16 open |

| open | [Browser Control 1057 Settings Inventory](workpacks/browser-control-1057-settings-inventory.md) | 630,266 | 0/0 checked; 0 open |
| open | [Browser Control Coverage Matrix](workpacks/browser-control-coverage-matrix.md) | 14,710 | 0/0 checked; 0 open |
| open | [Browser Control Schema Proposal](workpacks/browser-control-schema-proposal.md) | 53,276 | 0/0 checked; 0 open |
| open | [Browser Policy Questionnaire Forest V1](workpacks/browser-policy-questionnaire-forest-v1.md) | 29,535 | 0/0 checked; 0 open |
| open | [Browser Policy Settings Catalog](workpacks/browser-policy-settings-catalog.md) | 47,158 | 0/0 checked; 0 open |
| open | [Managed Unmanaged Browser](workpacks/managed-unmanaged-browser.md) | 32,773 | 0/0 checked; 0 open |
