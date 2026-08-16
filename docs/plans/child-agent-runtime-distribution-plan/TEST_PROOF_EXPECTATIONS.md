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

Use the subset relevant to the selected workpack. Do not run a full package matrix when the selected workpack is docs-only or platform-specific.

```bash
# Shared child package/runtime/setup-trust contracts
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- child
npm run build --workspace @ocentra-parent/child-runtime-domain
npm run test --workspace @ocentra-parent/child-runtime-domain

# Platform package artifacts; run only for the selected platform/package workpack
npm run release:package:windows
npm run release:package:macos
npm run release:package:linux
npm run release:package:android
npm run release:package:ios

# Existing mobile child proof anchors
npm run test:child-android-protocol-package-lifecycle-proof
npm run test:child-android-permission-capability-proof
npm run test:child-android-device-proof-artifact-gate
npm run test:child-ios-entitlement-capability-proof
npm run test:child-android-storage-protocol-capability-proof
npm run test:child-android-service-protocol-capability-proof

# Runtime/protocol scope only when selected workpack touches service/protocol behavior
cargo test -p ocentra-parent-agent-service child
cargo test -p ocentra-parent-agent-protocol child

# Architecture scope: start with touched files; expand only when the workpack requires it
npm run lint:architecture -- --files packages/schema-domain packages/child-runtime-domain crates/agent-service crates/agent-protocol scripts/release scripts/test docs/plans/child-agent-runtime-distribution-plan
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Command ownership notes

- `crates/schema` or another neutral Rust-owned crate owns canonical child package/runtime/platform capability/device-owner/managed-profile/supervision/setup-trust-handoff shapes when contracts cross package/crate/app/plan boundaries.
- `packages/schema-domain` is temporary generated-validation or edge-decoder scope only where TypeScript still consumes Rust-owned contracts during migration.
- `packages/child-runtime-domain` is package-boundary metadata/helper scope only; shared child runtime contracts live in Rust-owned schema surfaces.
- `scripts/release/*` prove artifact build/checksum/signing-package outputs only for the selected platform.
- `scripts/test/child-*` proof scripts prove the exact named proof mode only. Debug APK, simulator, manifest, scaffold, and package-local bridge proof must not be upgraded into real-device, store, device-owner, managed-profile, supervision, runtime transport, or enforcement proof.
- `crates/agent-service` and `crates/agent-protocol` are service/protocol proof only when the selected workpack names runtime or wire behavior.
- Setup, device-trust, account, policy, enforcement, notification, portal, LAN, remote, payment, and data-custody are sibling/consumer scopes. Run them only when the selected workpack explicitly touches the handoff.

## Child distribution E2E meaning

Do not use one proof family to claim the whole child distribution path. For this plan, E2E has separate meanings:

```text
artifact E2E: selected release script -> package artifact -> artifact manifest/checksum path.
signing/checksum E2E: package artifact -> checksum/SBOM/signing/notarization/store state -> explicit unsigned/debug/manual-required labels.
install/lifecycle E2E: package artifact -> install/update/start/stop/reboot/uninstall lifecycle proof on the selected platform.
runtime/service E2E: installed child package -> service/process lifecycle -> health/readiness/degraded state.
respawn/supervision E2E: service crash/stop/reboot case -> platform restart/supervision policy -> backoff/loop guard -> health state.
uninstall/revocation E2E: authorized parent/household/device request -> package removal/revocation path -> cleanup/residual-state proof.
device-owner/managed-profile/supervision E2E: platform enrollment/provisioning state -> capability/limitation proof -> manual-required gaps where unsupported.
setup-device-trust handoff E2E: setup-trust request -> child install target and trust refs -> package/distribution acceptance or blocked/manual-required state.
release gate E2E: all selected proof roots -> WP11 aggregate -> no-claim boundaries and remaining manual-required rows.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Every child distribution proof slice must preserve both product-safe logging and local harness logging.

Product/runtime-safe logging:

```text
redact account tokens, pairing secrets, signing keys, certificate private material, provisioning secrets, device serials when not needed, child private activity payloads, and store account identifiers
log artifact path/ref, checksum ref, signing state, platform, OS/version where safe, install state, service state, respawn state, uninstall/revocation state, setup trust handoff ref, degraded/manual-required reason, and audit/proof ref
separate artifact-build, install, runtime, respawn, uninstall, device-owner/managed-profile/supervision, setup handoff, and release-gate states
never treat package logs, setup logs, parent-client logs, or portal logs as child runtime distribution proof by themselves
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, owner module, platform, artifact kind, exit code, result, artifact pointer, diagnostics summary, platform note, manual-required note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## Expected proof focus by workpack

```text
WP01 scope and setup/trust handoff boundary
WP02 Windows service package, install/service lifecycle, signing state, and restart limits
WP03 macOS service package, launchd/notarization state, and lifecycle limits
WP04 Linux package, service-manager state, signing/repo state, and lifecycle limits
WP05 Android package, permission/platform status, device-owner/managed-profile gaps, and debug/store split
WP06 iOS capability package, provisioning/supervision state, launch-availability truth, recovery non-claim, and manual-required gaps
WP07 managed service restart/supervision state
WP08 parent-approved removal, revocation, cleanup, and residual-state proof
WP09 signing/store/platform/device-owner matrix
WP10 setup/trust typed handoff contract
WP11 proof/CI/release gate aggregation
```

## Required negative states

```text
package build is not runtime readiness
install is not readiness proof
runtime health is not respawn proof
respawn is not uninstall resistance
uninstall resistance is not hidden persistence
platform scaffold is not platform support
Android debug APK proof is not device-owner or managed-profile proof
iOS simulator/provisioning proof is not background-service or supervision parity
parent client proof cannot close child runtime rows
setup journey cannot close package rows
manual-required states remain visible
```

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof are complete for the selected slice.
- Do not store proof inventories inside this plan folder.
- Do not claim feature completeness until the relevant E2E tier above is explicitly proven or blocked.
