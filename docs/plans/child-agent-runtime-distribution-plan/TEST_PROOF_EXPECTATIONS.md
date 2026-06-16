<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Test Proof Expectations

## Proof root

```text
output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/
```

## Common command families

```bash
npm run release:package:windows
npm run release:package:macos
npm run release:package:linux
npm run release:package:android
npm run release:package:ios
npm run test:child-android-protocol-package-lifecycle-proof
npm run test:child-android-permission-capability-proof
npm run test:child-android-device-proof-artifact-gate
cargo test -p ocentra-parent-agent-service
cargo test -p ocentra-parent-agent-protocol
npm run lint:architecture -- --files crates/agent-service crates/agent-protocol scripts/release docs/plans/child-agent-runtime-distribution-plan
```

## Expected proof focus by workpack

```text
WP01 scope and setup/trust handoff boundary
WP02 Windows service package and lifecycle proof
WP03 macOS service package and lifecycle proof
WP04 Linux package and service-manager state
WP05 Android package and platform status
WP06 iOS capability package and provisioning status
WP07 managed service restart/supervision state
WP08 parent-approved removal and revocation state
WP09 signing/store/platform matrix
WP10 setup/trust handoff contract
WP11 proof/CI/release gate
```

## Required negative states

```text
package build is not runtime readiness
install is not readiness proof
platform scaffold is not platform support
parent client proof cannot close child runtime rows
setup journey cannot close package rows
manual-required states remain visible
```

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof are complete for the selected slice.
- Do not store proof inventories inside this plan folder.
