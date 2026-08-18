<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack unless the assignment is a plan-wide audit.
> Proves: workpack routing only.
> Does not prove: child runtime readiness, package readiness, device trust, setup readiness, or PR readiness.
> Proof rule: update completion only after source, tests, proof, checklist, and required review gates agree.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Workpack Index

Live source audit basis: committed code at `a60d4f593a182e48fc6bc897c0d1417be54d3b05`, before final consolidation rebase. Status below is source-routing truth, not completion.

| Status | Workpack | Production source gap | Expected test-source gap | Runtime/caller state | Implementation dependency |
| --- | --- | --- | --- | --- | --- |
| route-only; not product completion | [WP01 Child Agent Scope And Route Boundary](workpacks/01-child-agent-scope-and-route-boundary.md) | None; keep the ownership route aligned. | Route/index consistency only. | Not runtime. | None. |
| source partial | [WP02 Child Windows Service Package](workpacks/02-child-windows-service-package.md) | Trusted startup, authenticated ingress, external health, and canonical child-owned package source identities. | Child service startup/readiness plus child-labelled elevated lifecycle/respawn. | Installed service has no trust source or production client. | Child WP10 reviewed implementation. |
| source partial | [WP03 Child macOS Service Package](workpacks/03-child-macos-service-package.md) | Canonical child plist/source identity, signing/notarization, trusted startup, health, and lifecycle completion. | Real-host launchd/signing/restart/disable/uninstall/health. | Binary remains trust-manual-required and externally unreachable. | Child WP10 reviewed implementation. |
| source partial | [WP04 Child Linux Service Package](workpacks/04-child-linux-service-package.md) | Canonical child unit/source identity, fail-closed service lifecycle, trusted startup, health, signing/feed, and cleanup. | Child-labelled package plus real-host health/crash/restart/remove/distro tests. | systemd starts an unbound, externally unreachable service. | Child WP10 reviewed implementation. |
| source partial | [WP05 Child Android Agent Package](workpacks/05-child-android-agent-package.md) | Current-trust JNI startup, authenticated ingress, usable health, device-owner/managed-profile, and removal integration. | Fail-closed/no-trust, current-trust, foreground lifecycle, ingress, removal, and device authority. | Binder health is local; transport is `NOT_IMPLEMENTED`. | Child WP10 reviewed implementation. |
| source correction pending | [WP06 Child iOS Capability Package](workpacks/06-child-ios-agent-capability-package.md) | Actual project/bundle/scheme/app/release identity is still parent-labelled. | Child identity build/smoke and simulator/device limit tests. | Capability-only; no daemon/runtime parity. | None; first independent source packet. |
| source incomplete | [WP07 Child Managed Service Respawn](workpacks/07-child-managed-service-respawn.md) | Health-aware lifecycle/supervision, bounded restart/backoff, deliberate-stop, teardown, and platform callbacks. | Kill/reboot/manager-restart/disable/teardown/loop-guard by platform. | Static declarations only; no live observer. | Child WP02-WP06 and WP10 reviewed implementation. |
| source partial / caller missing | [WP08 Child Parent Authorized Uninstall](workpacks/08-child-parent-authorized-uninstall.md) | Production authority caller, platform cleanup callbacks, and durable cleanup receipts. | Authority mismatch/replay/restart and platform cleanup/idempotency. | Removal APIs have no production caller. | Account WP08, Child WP10, and Child WP07 reviewed implementation. |
| source partial / integration missing | [WP09 Child Signing Store Device Owner Matrix](workpacks/09-child-signing-store-device-owner-matrix.md) | Live WP10 handoff consumption plus non-Windows signing/store/update ownership. | Updater handoff/install/restart and platform signing/store/device-owner. | Windows updater CLI does not consume the handoff projection. | Child WP02-WP06 and WP10 reviewed implementation. |
| source partial / first runtime packet | [WP10 Setup Device Trust Handoff](workpacks/10-setup-device-trust-handoff.md) | Trusted startup adapter, authenticated ingress, external health, durable handoff delivery/replay, and live updater consumer. | Trust/currentness, ingress, health, replay/expiry, updater callback, crash/restart. | Projection has no caller; shipped startup has no trust source. | Device Trust WP01 reviewed implementation. No reverse edge to Setup WP07. |
| source missing | [WP11 Proof CI Release Gate](workpacks/11-proof-ci-release-gate.md) | Executable aggregate release gate. | Negative fixture and release-blocker coverage. | No workflow consumes one authoritative aggregate result. | Child WP01-WP10 reviewed implementation; normal completion still requires all strict gates. |

## Source execution order

```text
WP01 route

WP06 child iOS identity       Device Trust WP01
          \                  /
           \                v
            +-----------> WP10
                            |
              +-------------+-------------+-------------+
              v             v             v             v
             WP02          WP03          WP04          WP05
               \             |             |             /
                +------------+------+------+------------+
                                    v
                                   WP07
                                  /    \
                    Account WP08 v      v
                                WP08   WP09
                                  \    /
                                   v  v
                                   WP11
```

Implementation-only dependencies order source packets; they do not change normal `READY`, validation, proof, or `DONE` requirements.

## Selection rules

- Select WP06 or WP10 first. WP10 is legal only when graph inspection confirms the Device Trust WP01 reviewed-implementation gate.
- Select WP02-WP05 only after WP10's reviewed implementation roots exist.
- Select WP07 only after the platform package/runtime source roots exist.
- Select WP08 only after Account authority, WP10, and WP07 source roots exist.
- Select WP09 only after canonical platform identities and WP10 source exist.
- Select WP11 last.
- Do not reuse parent-client proof or treat a proof/checklist count as source completion.
