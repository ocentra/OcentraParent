<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Test Proof Expectations`
> Kind: test-source, command, and proof selector.
> Read when: the selected source packet is complete and the program enters the test-source or validation wave.
> Stop rule: do not run tests during the current source-routing packet.
> Proves: expected coverage only; no test or proof result is claimed here.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Test Proof Expectations

Current phase: WP01 route-boundary and WP06 child iOS capability retained proofs have been executed at their canonical four-file proof roots. The WP06 Windows result records the real XCTest as platform-unavailable/manual-required rather than promoting it to a pass. Other workpacks remain routed gaps; add their coherent test source only after the owning production source exists.

## Expected test-source gaps

| Workpack | Required test-source outcomes |
| --- | --- |
| WP01 | Implemented at `tests/repo-tooling/child-agent-runtime-distribution-route.test.mjs`: generated graph/code-map behavior rejects parent-client ownership, product roots, runtime/package promotion, and wrong completion evidence. `scripts/test/child-agent-scope-and-route-boundary-proof.mjs` executed the focused test and graph validation and wrote the canonical four-file proof root. |
| WP02 | Current-trust startup, missing/stale trust fail-closed, authenticated ingress, external health, and child-labelled elevated Windows install/start/stop/restart/uninstall/respawn. |
| WP03 | Child identity, launchd install/start/health/restart/disable/uninstall, signing/notarization inputs and rejection states, and cleanup on a real macOS host. |
| WP04 | Child identity, `.deb` metadata, service health, crash/restart, deliberate stop, disable/remove/purge, cleanup, distro baseline, and signing/feed rejection states on Linux. |
| WP05 | JNI startup without trust remains manual-required; current trust reaches ready; foreground restart/stop, authenticated ingress, health, native load/ABI failure, removal, and device-owner/managed-profile manual-required behavior. |
| WP06 | Implemented by the real Node/XCTest child identity harness and runner tests: actual child project/product/bundle/scheme/artifact identity plus simulator/host outcome, capability-limit, no-daemon, provisioning, signing, supervision, background, recovery, and parent-parity non-claims. Enforcer run `ocentra-parent.child-ios-entitlement-capability-proof-20260902102636-1c1154d5` wrote the canonical retained bundle. Physical-device and Apple authority evidence remains manual-required and unexecuted. |
| WP07 | Platform manager kill/reboot/restart/deliberate-stop/disable/teardown behavior, bounded backoff/loop guard, and health-aware state transitions. |
| WP08 | Current Account authority acceptance, wrong actor/target/action/replay/generation rejection, restart durability, platform cleanup callback idempotency, residual custody, and cleanup receipts. |
| WP09 | Manifest/signature/hash rejection, installer result/reboot/restart behavior, WP10 handoff consumption, scheduler/retry state, and platform-specific signing/store/device-owner truth. |
| WP10 | Current Device Trust source binding, stale/revoked/missing binding rejection, startup recovery, authenticated ingress, external health, durable handoff replay/expiry, updater callback, and crash/restart recovery. |
| WP11 | Aggregate negative fixtures for every missing identity, trust, ingress, health, lifecycle, removal, updater/handoff, signing/store, mobile manual-required, and proof/CI condition. |

The existing Android bridge test that expects `Ready` from `ChildRuntimeAndroidBridge::start` is stale against fail-closed startup: the bridge supplies no trust source. It must be corrected during the WP05/WP10 test-source wave, not used as completion evidence now.

## Validation families after test source exists

Choose the smallest family for the selected workpack; do not run the full package matrix by habit.

```text
shared child runtime: ocentra-child-runtime focused unit/integration/contract tests
Android JNI: ocentra-child-runtime-android-bridge focused integration tests plus Android instrumentation
updater/handoff: ocentra-parent-agent-maintenance focused unit/contract/integration tests
Windows: child-labelled MSI lifecycle harness on an elevated Windows host
macOS: child package/launchd/signing/notarization lifecycle harness on a macOS host
Linux: child .deb/systemd lifecycle harness on the declared distro baseline
iOS: child project identity/build/simulator capability harness; physical-device rows remain device-proof-required
aggregate: executable WP11 negative-fixture gate across all reviewed child workpack outputs
```

Focused source checks for the two present slices:

```text
node --test tests/repo-tooling/child-agent-runtime-distribution-route.test.mjs
node --test tests/repo-tooling/child-ios-entitlement-capability-proof-runner.test.mjs
node --test platforms/ios/tests/child_capability_identity.test.mjs
```

The third command is a real XCTest invocation on macOS/Xcode and an explicit host-blocked skip elsewhere. WP01 retained proof is regenerated only by `node scripts/test/child-agent-scope-and-route-boundary-proof.mjs`; WP06 retained proof is regenerated through `npm run test:child-ios-entitlement-capability-proof`.

Required architecture scope when Rust/Java/Swift/JavaScript/TypeScript source is touched remains the nearest owning crate/app/script paths. Expand to repo-wide validation only after the complete source and test waves.

## Proof root

```text
output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/
```

Regenerate proof only after production source, expected test source, and focused validation agree. Historical proof and `test-results` from a different identity/runtime path are review inputs only.

## E2E meanings

```text
artifact E2E: canonical child source identity -> package artifact -> manifest/checksum.
trusted startup E2E: current Device Trust binding -> shipped service startup -> fail-closed readiness.
ingress/health E2E: authenticated product caller -> child command boundary -> externally observable health/result.
lifecycle E2E: installed package -> start/stop/crash/reboot/manager restart -> bounded state and teardown.
removal E2E: current Account authority -> child revocation -> platform cleanup callback -> durable receipt/residual custody truth.
update E2E: setup-owned handoff -> durable delivery/replay guard -> updater -> installer outcome/restart -> child package state.
mobile authority E2E: platform provisioning/device-owner/supervision state -> capability result or explicit manual-required state.
release E2E: executable WP11 gate -> all selected code/tests/proof/CI inputs -> fail-closed release decision.
```

No one tier closes another.

## Structured logging expectations

- Redact account tokens, pairing/trust secrets, signing keys, device serials, child private activity, and store identities.
- Retain safe workpack, platform, artifact, trust state, lifecycle state, removal state, updater/handoff state, run id, exit/result, and artifact pointer fields.
- Keep artifact, startup, ingress/health, lifecycle, removal, mobile authority, updater/handoff, and release-gate states separate.
- Store raw output by artifact pointer; keep plan docs compact.

## Failure conditions

- Writing tests that assert the current incomplete source behavior as success.
- Using mocks/fakes to replace the real authority, filesystem, service manager, installer, JNI, or platform boundary when the workpack requires that boundary.
- Treating contract/schema, package build, debug APK, simulator launch, static restart declarations, or proof folders as runtime completion.
- Running broad tests before the coherent production and test source packet exists.
- Marking DONE or PR_READY before code, tests, validation, proof, checklist, review, CI, and merge gates agree.
