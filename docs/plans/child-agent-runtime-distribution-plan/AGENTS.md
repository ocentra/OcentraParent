<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `AGENTS.md`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX or FEATURE_ROUTE_INDEX selects the plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_STATE.md, WORKPACK_INDEX.md, TEST_PROOF_EXPECTATIONS.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Plan Agent Route

Canonical scope: child-agent runtime distribution. This plan owns child Windows, macOS, Linux, Android, and iOS package distribution, managed respawn where the platform supports it, tamper/uninstall proof, signing/store/device-owner matrix, and setup-device-trust handoff. Parent client distribution stays in `parent-client-runtime-distribution-plan`.

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/child-agent-runtime-distribution-plan`.

## High-density execution contract

- Task: work only the assigned slice for this plan.
- Route first from `PLAN_STATE.md`.
- Choose exactly one workpack.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
- Do not inspect sibling plans unless the selected workpack names a handoff.
- Proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
- Do not claim child tamper/uninstall, device-owner, or respawn behavior from parent client proof.

## Ownership, Import, And Boundary Contract

This plan owns packaging and distribution proof for child runtime artifacts. It does not own the parent client package, the setup journey, account identity, policy behavior, AI behavior, enforcement adapters, notification delivery, portal display, LAN protocol behavior, or data custody.

Module roles:

```text
crates/schema or the owning Rust crate: canonical shared child package, child runtime, platform capability, device-owner, managed-profile, supervision, artifact, signing, setup-trust-handoff, release-gate, and route/action/read-model DTO shapes when those shapes cross package, crate, app, or plan boundaries.
schema-domain: temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
child-runtime-domain: child runtime package-boundary metadata/helper surface. Shared child runtime contracts live in Rust-owned schema surfaces.
agent-protocol and agent-service: runtime/protocol proof only when the selected workpack names child runtime, service health, package lifecycle protocol, or service-manager proof.
scripts/release: artifact build/checksum/signing-packaging proof only. Package scripts do not prove install, runtime, respawn, uninstall, setup, transport, or platform readiness by themselves.
setup-install-provisioning-plan: setup journey owner; this plan consumes only typed setup-to-child-install handoff state.
device-trust-bootstrap-plan: trusted-device bootstrap, local sealed trust, and device trust material owner.
parent-client-runtime-distribution-plan: parent client artifact/distribution owner. Parent proof cannot close child artifact rows.
policy, enforcement, AI, portal, notification, LAN, remote, account, payment, and data-custody plans: sibling behavior, display, transport, identity, billing, or custody owners only. They must not re-own child package/distribution truth.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
Rust-owned canonical child-runtime/package/platform-capability/setup-trust-handoff/artifact/signing/release-gate shapes plus generated DTOs or temporary edge decoders
neutral event/evidence/logging/protocol primitives
child-runtime-domain package-info and approved metadata helpers
agent-protocol/agent-service public surfaces only when runtime/protocol proof is selected
release script outputs and artifact manifests when package proof is selected
```

Forbidden direct imports and claims:

```text
parent-client runtime internals used as child package proof
setup UI or parent bootstrap internals used as child install/package proof
policy, enforcement, AI, portal, notification, LAN, remote, account, payment, or data-custody runtime behavior imported into this plan's package proof
Android debug APK proof upgraded into device-owner, managed-profile, privileged capability, runtime background, transport, or store distribution proof
iOS simulator or provisioning proof upgraded into background-service parity or supervision parity
package build/checksum/signing proof upgraded into install, runtime health, respawn, uninstall, transport, policy, enforcement, or setup readiness
platform unsupported/manual-required states hidden or collapsed into ready states
```

If child distribution work needs setup, device trust, policy, enforcement, notification, portal, LAN, remote, account, or data custody behavior, it must use typed evidence, commands, events, requests, read models, artifact manifests, and proof handoffs. If a shape is used by multiple feature owners, place or consume it through `crates/schema` or another neutral Rust-owned boundary. Use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete. Do not solve cross-plan behavior by importing another feature's runtime internals.

## Research gate

Before DONE or PR_READY, inspect only the selected slice of:

- the current child-service/runtime implementation in the repo
- the package scripts for Windows, macOS, Linux, Android, and iOS distribution
- platform docs for Windows service / restart semantics
- platform docs for macOS launchd and notarization
- platform docs for Linux service managers and package signing
- Android package, service, device-owner, and managed-profile guidance
- iOS distribution and supervision/provisioning limits

## Decision tree

| If the task is about...                               | Open                                                      |
| ----------------------------------------------------- | --------------------------------------------------------- |
| Child scope correction and setup-device-trust handoff | `workpacks/01-child-agent-scope-and-route-boundary.md`    |
| Child Windows package                                 | `workpacks/02-child-windows-service-package.md`           |
| Child macOS package                                   | `workpacks/03-child-macos-service-package.md`             |
| Child Linux package                                   | `workpacks/04-child-linux-service-package.md`             |
| Child Android package                                 | `workpacks/05-child-android-agent-package.md`             |
| Child iOS capability package                          | `workpacks/06-child-ios-agent-capability-package.md`      |
| Managed service respawn                               | `workpacks/07-child-managed-service-respawn.md`           |
| Parent-authorized uninstall                           | `workpacks/08-child-parent-authorized-uninstall.md`       |
| Signing/store/device-owner matrix                     | `workpacks/09-child-signing-store-device-owner-matrix.md` |
| Setup-device-trust handoff                            | `workpacks/10-setup-device-trust-handoff.md`              |
| Proof, CI, and release gate                           | `workpacks/11-proof-ci-release-gate.md`                   |

## Architecture decisions

- Child agent distribution is separate from parent client distribution.
- Windows, macOS, and Linux rows may support managed respawn and uninstall resistance through the platform service manager or package manager.
- Android rows may support device-owner or managed-profile states only where platform setup and proof allow them.
- iOS rows must stay honest about supervision, provisioning, and background-service limits.
- Signing, store, and device-owner claims must be explicit per artifact.
- Parent-authorized uninstall is a platform and custody claim, not a hidden persistence claim.

## Handoffs

- `setup-install-provisioning-plan` owns the setup journey and device-trust handoff into install state.
- `parent-client-runtime-distribution-plan` owns the parent client distribution boundary.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap and local sealed trust.
- `child-agent-local-service`/child runtime owners own local-service runtime behavior; this plan packages and proves distribution boundaries.

## Failure conditions

- Do not claim parent client parity from child package proof.
- Do not hide platform-specific device-owner or provisioning gaps.
- Do not claim respawn where the platform cannot prove it.
- Do not conflate uninstall resistance with hidden persistence.
- Do not hide manual-required gaps.
- Do not mark package readiness from artifact build, checksum, package script, setup UI, or parent client proof alone.
