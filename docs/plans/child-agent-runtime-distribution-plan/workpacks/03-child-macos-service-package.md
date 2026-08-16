# Workpack 03 - Child macOS Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `03-child-macos-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child macOS package, launchd lifecycle, notarization, and uninstall proof boundary.

## Owns

- macOS child package shape
- launchd lifecycle and restart state
- notarization and signing state for the child artifact
- uninstall, disable, and removal behavior on macOS

## Must prove

- the package launches through the macOS service boundary
- restart or recovery behavior is honest for the platform
- signing and notarization state are explicit per artifact
- provisioning or entitlement gaps are surfaced as manual-required states
- no child background-service claim exceeds macOS limits

## Failure conditions

- persistent service behavior is claimed without macOS proof
- notarization or provisioning gaps are hidden
- uninstall and disable behavior is not audited
- parent-client parity is implied from the child slice

## Execution truth

Rust/shared owner truth remains upstream. This packet uses `schema-domain` only as a temporary thin/generated edge proof contract surface for the child macOS package boundary.

Production code is drafted in this pass: the macOS package builder and launchd manifest now target the child-agent executable and child package identity. Tests, validation, and proof are deferred; the proof-root and proof-runner references do not establish completion in this phase.

## Intended source states (unvalidated)

- child artifact mode is explicit `launchd-pkg-script`
- child artifact state is explicit `pkg-script-defined`
- launchd service boundary is explicit `launchd-boundary-scripted`
- `RunAtLoad` is present in the LaunchDaemon plist
- `KeepAlive` is present in the LaunchDaemon plist
- signing state is explicit `unsigned`

## Manual-required states

- package install on a real macOS host
- launchd runtime health on a real macOS host
- restart or recovery beyond the `KeepAlive` declaration
- Apple signing identity and signed entitlements
- Apple notarization and stapled ticket artifacts
- disable, uninstall, removal, and cleanup behavior

## Validations

- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/child-macos-service-package-proof.test.ts`
- `cmd /c npm run test:child-macos-service-package-proof`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/child-macos-service-package-proof.ts packages/schema-domain/tests/proof/child-macos-service-package-proof.test.ts scripts/test/child-macos-service-package-proof.mjs`
- `cmd /c npm run type-check --workspace @ocentra-parent/schema-domain`
- `cmd /c npx eslint packages/schema-domain/src/child-macos-service-package-proof.ts packages/schema-domain/tests/proof/child-macos-service-package-proof.test.ts scripts/test/child-macos-service-package-proof.mjs`

## No-claim boundary

WP03 does not claim:

- real macOS install success
- real launchd service start or steady-state health
- restart or recovery proof from `KeepAlive` alone
- Apple `codesign`, `productsign`, signed entitlements, notarization, or stapled ticket artifacts
- disable, uninstall, removal, or cleanup execution on a real macOS host
- parent-client parity or hidden background-service persistence

## Closure truth

WP03 remains production-code drafted. Tests, validation, signing, and retained proof are deferred; this pass does not close the package or installed-runtime boundary.
