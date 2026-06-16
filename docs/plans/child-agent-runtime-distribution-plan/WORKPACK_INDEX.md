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

| Status | Workpack | Boxes | Proof root |
| --- | --- | ---: | --- |
| open | [WP01 Child Agent Scope And Route Boundary](workpacks/01-child-agent-scope-and-route-boundary.md) | 0/10 | `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/` |
| open | [WP02 Child Windows Service Package](workpacks/02-child-windows-service-package.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/` |
| open | [WP03 Child macOS Service Package](workpacks/03-child-macos-service-package.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/03-child-macos-service-package/` |
| open | [WP04 Child Linux Service Package](workpacks/04-child-linux-service-package.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/` |
| open | [WP05 Child Android Agent Package](workpacks/05-child-android-agent-package.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/` |
| open | [WP06 Child iOS Capability Package](workpacks/06-child-ios-agent-capability-package.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/` |
| open | [WP07 Child Managed Service Respawn](workpacks/07-child-managed-service-respawn.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/07-child-managed-service-respawn/` |
| open | [WP08 Child Parent Authorized Uninstall](workpacks/08-child-parent-authorized-uninstall.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/` |
| open | [WP09 Child Signing Store Device Owner Matrix](workpacks/09-child-signing-store-device-owner-matrix.md) | 0/12 | `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/` |
| open | [WP10 Setup Device Trust Handoff](workpacks/10-setup-device-trust-handoff.md) | 0/10 | `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/` |
| open | [WP11 Proof CI Release Gate](workpacks/11-proof-ci-release-gate.md) | 0/14 | `output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/` |

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
