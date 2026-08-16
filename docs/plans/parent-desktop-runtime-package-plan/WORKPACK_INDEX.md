<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: package readiness, signing readiness, store readiness, setup readiness, or PR readiness.
> Proof rule: update counts/status only after matching proof artifacts exist.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Workpack Index

The folder path remains `parent-desktop-runtime-package-plan`, but the canonical scope is parent client runtime distribution: web, desktop, Android parent, and iOS parent. Child agent runtime distribution belongs to `child-agent-runtime-distribution-plan`.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Proof root |
| --- | --- | ---: | --- |
| complete | [WP01 Parent Client Scope And Route Boundary](workpacks/01-parent-client-scope-and-route-boundary.md) | 10/10 | `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/` |
| complete | [WP02 Parent Web Portal Distribution](workpacks/02-parent-web-portal-distribution.md) | 12/12 | `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/` |
| complete | [WP03 Parent Desktop Shell Package](workpacks/03-parent-desktop-shell-package.md) | 12/12 | `output/parent-client-runtime-distribution-plan-proof/03-parent-desktop-shell-package/` |
| complete | [WP04 Parent Android Package](workpacks/04-parent-android-package.md) | 12/12 | `output/parent-client-runtime-distribution-plan-proof/04-parent-android-package/` |
| open | [WP05 Parent iOS Package](workpacks/05-parent-ios-package.md) | 0/12 | `output/parent-client-runtime-distribution-plan-proof/05-parent-ios-package/` |
| complete | [WP06 Parent Local Service Route Bridge](workpacks/06-parent-local-service-route-bridge.md) | 12/12 | `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/` |
| open | [WP07 Parent Client Signing Store Matrix](workpacks/07-parent-client-signing-store-matrix.md) | 0/12 | `output/parent-client-runtime-distribution-plan-proof/07-parent-client-signing-store-matrix/` |
| complete | [WP08 Parent Client Update Rollback](workpacks/08-parent-client-update-rollback.md) | 12/12 | `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/` |
| complete | [WP09 Parent Client Launch Smoke Matrix](workpacks/09-parent-client-launch-smoke-matrix.md) | 12/12 | `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/` |
| open | [WP10 Setup Handoff Contracts](workpacks/10-setup-handoff-contracts.md) | 0/10 | `output/parent-client-runtime-distribution-plan-proof/10-setup-handoff-contracts/` |
| open | [WP11 Proof CI Release Gate](workpacks/11-proof-ci-release-gate.md) | 0/14 | `output/parent-client-runtime-distribution-plan-proof/11-proof-ci-release-gate/` |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP07 -> WP08 -> WP09 -> WP10 -> WP11
```

## Dependency rules

```text
WP01 fixes scope and route boundary before package work.
WP02 handles web portal distribution.
WP03 handles desktop shell/package only.
WP04/WP05 handle parent mobile package claims separately.
WP06 handles local service route bridge without claiming setup completion.
WP07 handles signing/store/notarization matrix before release claims.
WP08 handles update/rollback/checksum/SBOM.
WP09 handles launch smoke by artifact/platform.
WP10 handles setup handoff contract only.
WP11 is last and consumes all previous proof roots.
```

## Status rules

- If a workpack text says proof is recorded but this index says open, keep the row open until the proof root, checklist row, and PLAN_STATE are aligned.
- Do not raise status from scaffold, source script presence, launch smoke, CI success, preview build, or package metadata alone.
- Do not use one platform artifact to imply another platform artifact.

## Do not select

Do not implement child agent runtime distribution, setup journey state machine, device trust, account identity, payment behavior, policy behavior, remote access, data custody, child capture/enforcement adapters, or portal shell UX in this plan.
