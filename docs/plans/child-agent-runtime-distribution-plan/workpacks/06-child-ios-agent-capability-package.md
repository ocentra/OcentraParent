# Workpack 06 - Child iOS Capability Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `06-child-ios-agent-capability-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: own the canonical child iOS capability-package identity, provisioning limits, and manual-required boundary.

Status: canonical source identity corrected; expected tests, workflows, signing, device/store authority, and proof remain open; not complete.

## Owns

- actual iOS child project, product, scheme, bundle, application, release artifact, and smoke identity;
- capability-only lifecycle truth;
- provisioning, signing, supervision, launch, recovery, and store limits;
- manual-required states for unsupported child service behavior.

## Live source truth

Rust-owned capability/limit contracts, generated/thin TypeScript edges, contract tests, and a focused proof runner exist. They correctly model capability-only, no-daemon, no-parity, provisioning/signing/supervision/manual-required states.

Reviewed source at `c71becbcfd4f07eb98a118f10dbf261320f6b54e` now matches the child route: the checked-in project, target, scheme, app folder/name, product name, bundle identifier, release input, and simulator artifact use `OcentraChildAgent`, `ca.ocentra.child.agent`, and child-owned artifact names. Smoke/workflow consumers, Apple signing/provisioning, physical-device launch, TestFlight/App Store authority, expected tests, and retained proof have not been completed.

## Required production source outcome

- one canonical child iOS project/product/scheme/bundle/application/release identity;
- release and smoke inputs that refer to that same child identity;
- preserved capability-only, no-daemon, no-persistent-service, recovery-not-implemented, provisioning/signing/supervision/device-proof-required boundaries;
- no reuse of parent-client identity or proof.

WP06 has no implementation dependency. Its first source packet is reviewed, while its expected-test, validation, proof, PR, and normal DONE gates remain open.

## Expected test-source gap

- reject parent project/product/scheme/bundle/artifact identity;
- build and launch the canonical child capability app in the simulator path;
- retain explicit device launch, provisioning, signing, TestFlight/App Store, supervision, background execution, and recovery limits;
- prove no daemon, hidden control, persistent service, or parent-client parity claim.

## Failure conditions

- marking WP06 complete from the capability contract while the actual app identity is still the parent identity;
- using simulator or contract proof to claim physical-device, signing, store, supervision, background-service, or recovery parity;
- changing the identity while weakening any manual-required/no-claim state.

Historical proof under `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/` remains review input. Regenerate it only after source and expected test source are aligned.
