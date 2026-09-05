# Workpack 06 - Child iOS Capability Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `06-child-ios-agent-capability-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: own the canonical child iOS capability-package identity, provisioning limits, and manual-required boundary.

Status: canonical source identity, expected child capability test source, focused validation, and the exact retained capability-boundary proof are complete. macOS/Xcode execution, Apple signing/provisioning, physical-device launch, TestFlight/App Store authority, and aggregate release completion remain manual-required or open.

## Owns

- actual iOS child project, product, scheme, bundle, application, release artifact, and smoke identity;
- capability-only lifecycle truth;
- provisioning, signing, supervision, launch, recovery, and store limits;
- manual-required states for unsupported child service behavior.

## Tracking boundary

WP06 owns the canonical iOS child package and capability-only truth. It does
not own Core Location foreground/background semantics or a tracking transport.
Tracking WP11/WP12 may add only their routed Swift adapter roots after Child
WP10 supplies trusted child startup/ingress and Tracking WP40 supplies trusted
durable tracking ingress. A local file/JSON handoff with no owner consumer is
not an implementation.

## Live source truth

Rust-owned capability/limit contracts, generated/thin TypeScript edges, contract tests, and a focused proof runner exist. They correctly model capability-only, no-daemon, no-parity, provisioning/signing/supervision/manual-required states.

`platforms/ios/tests/child_capability_identity.test.mjs` invokes the real `OcentraChildAgentUITests/ChildCapabilityIdentityUITests` XCTest on macOS/Xcode and records an explicit platform-unavailable/manual-required skip elsewhere. The XCTest verifies the launched child bundle/product identity and rendered capability-only/manual-required/no-claim states. The retained Windows result records that host skip as blocked evidence; it is not a macOS/Xcode pass.

`scripts/test/child-ios-entitlement-capability-proof.mjs` targets the graph-canonical proof stem `output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/`, records the actual XCTest pass/fail or explicit host skip, and retains `test-results/child-ios-entitlement-capability-proof/proof.json` for the existing proof-matrix contract. Enforcer run `ocentra-parent.child-ios-entitlement-capability-proof-20260902102636-1c1154d5` passed and wrote the exact four-file retained bundle while preserving the Windows host-blocked state.

Reviewed source at `c71becbcfd4f07eb98a118f10dbf261320f6b54e` matches the child route: the checked-in project, target, scheme, app folder/name, product name, bundle identifier, release input, and simulator artifact use `OcentraChildAgent`, `ca.ocentra.child.agent`, and child-owned artifact names. Expected test source and the host-aware retained proof are complete. Apple signing/provisioning, physical-device launch, TestFlight/App Store authority, and a macOS/Xcode execution result remain external/manual-required.

## Required production source outcome

- one canonical child iOS project/product/scheme/bundle/application/release identity;
- release and smoke inputs that refer to that same child identity;
- preserved capability-only, no-daemon, no-persistent-service, recovery-not-implemented, provisioning/signing/supervision/device-proof-required boundaries;
- no reuse of parent-client identity or proof.

WP06 has no implementation dependency. Its bounded capability-only source, test, validation, and retained-proof contract is complete. This does not satisfy Apple authority, physical-device, aggregate WP11, precommit, CI, PR, merge, or child release-readiness gates.

## Expected test-source state

- Present source rejects parent project/product/scheme/bundle/artifact identity at the proof boundary.
- Present Node/XCTest source builds and launches the canonical child capability app on a suitable macOS/Xcode simulator host; other hosts skip with an explicit manual-required reason.
- Present assertions retain explicit device launch, provisioning, signing, TestFlight/App Store, supervision, background execution, and recovery limits.
- Present assertions prove no daemon, hidden control, persistent service, external transport, or parent-client parity claim.
- Physical-device, signing, provisioning, store, supervision, background, and recovery evidence remains manual/external and open.

## Failure conditions

- marking WP06 complete from the capability contract while the actual app identity is still the parent identity;
- using simulator or contract proof to claim physical-device, signing, store, supervision, background-service, or recovery parity;
- changing the identity while weakening any manual-required/no-claim state.

Historical proof under `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/` remains review input only. Current proof is the exact four-file bundle under `output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/`. The legacy `test-results` JSON remains a compatibility artifact, not the canonical plan proof root.
