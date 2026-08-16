<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: child runtime readiness, package readiness, device trust, setup readiness, or PR readiness.
> Proof rule: update counts/status only after matching proof artifacts exist.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Workpack Index

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Proof root |
| --- | --- | ---: | --- |
| complete | [WP01 Child Agent Scope And Route Boundary](workpacks/01-child-agent-scope-and-route-boundary.md) | 10/10 | `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/` |
| code drafted / test-deferred | [WP02 Child Windows Service Package](workpacks/02-child-windows-service-package.md) | service composition/package code drafted; tests/validation/proof deferred | `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/` |
| code drafted / test-deferred | [WP03 Child macOS Service Package](workpacks/03-child-macos-service-package.md) | production code drafted; tests/validation/proof deferred | `output/child-agent-runtime-distribution-plan-proof/03-child-macos-service-package/` |
| code drafted / test-deferred | [WP04 Child Linux Service Package](workpacks/04-child-linux-service-package.md) | production code drafted; tests/validation/proof deferred | `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/` |
| code drafted / test-deferred | [WP05 Child Android Agent Package](workpacks/05-child-android-agent-package.md) | child identity/composition drafted; tests/validation/proof deferred | `output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/` |
| complete | [WP06 Child iOS Capability Package](workpacks/06-child-ios-agent-capability-package.md) | 12/12 | `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/` |
| code drafted / test-deferred | [WP07 Child Managed Service Respawn](workpacks/07-child-managed-service-respawn.md) | manager configuration retargeted; tests/validation/proof deferred | `output/child-agent-runtime-distribution-plan-proof/07-child-managed-service-respawn/` |
| code drafted / test-deferred | [WP08 Child Parent Authorized Uninstall](workpacks/08-child-parent-authorized-uninstall.md) | service revocation/audit boundary drafted; platform cleanup/tests/proof deferred | `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/` |
| code drafted / test-deferred | [WP09 Child Signing Store Device Owner Matrix](workpacks/09-child-signing-store-device-owner-matrix.md) | Windows manifest/checksum/signature consumption drafted; platform matrix/tests/proof deferred | `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/` |
| code drafted / test-deferred | [WP10 Setup Device Trust Handoff](workpacks/10-setup-device-trust-handoff.md) | typed handoff contract and package/update consumer drafted; tests/validation/proof deferred | `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/` |
| complete | [WP11 Proof CI Release Gate](workpacks/11-proof-ci-release-gate.md) | 14/14 | `output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/` |

## Default execution order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP07 -> WP08 -> WP09 -> WP10 -> WP11
```

## Dependency rules

```text
WP01 fixes child-agent scope before package work.
WP02-WP06 are platform/package-specific.
WP07 validates restart/supervision behavior.
WP08 validates parent-approved removal and revocation state.
WP09 validates signing/store/device-owner matrix.
WP10 links setup/device-trust handoff only.
WP11 is last and consumes all previous proof roots.
```

## Do not select

Do not reuse parent-client proof pointers. Do not implement parent setup journey, account identity, LAN protocol, policy logic, or data custody here.

Do not raise status/counts from package-script presence, checksum presence, parent client proof, empty proof directories, stale legacy proof paths, or manual-required rows.
