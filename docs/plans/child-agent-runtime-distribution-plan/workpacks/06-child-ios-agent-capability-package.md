# Workpack 06 - Child iOS Capability Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `06-child-ios-agent-capability-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child iOS capability package, provisioning limits, and manual-required proof boundary.

Current status: `complete`.

## Owns

- iOS capability package shape
- provisioning and supervision limits
- launch, availability, and recovery honesty on iOS
- manual-required states for unsupported child service behavior

## Must prove

- the iOS artifact is honest about what it can and cannot do
- provisioning and supervision limits are explicitly represented
- capability-only state is used when full service behavior is unavailable
- no hidden control or daemon claim exceeds iOS limits
- parent-client parity is not implied from the iOS slice

## Failure conditions

- background or persistent-service claims exceed iOS proof
- provisioning limits are hidden
- manual-required states are omitted
- the slice claims more than a capability package

## Execution truth

- The canonical shared WP06 contract/read-model owner is now Rust-first:
  - `crates/schema/src/child_ios_entitlement_capability_proof.rs`
  - `crates/schema/src/child_ios_entitlement_capability_proof_ts.rs`
- The remaining TypeScript surface is generated/thin only:
  - `packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts`
  - `packages/schema-domain/src/child-ios-entitlement-capability-proof.ts`
- Real tests now exist on both sides of the boundary:
  - `crates/schema/tests/contract/child_ios_entitlement_capability_proof.rs`
  - `packages/schema-domain/tests/proof/child-ios-entitlement-capability-proof.test.ts`
- The focused proof runner is `scripts/test/child-ios-entitlement-capability-proof.mjs`.
- The canonical proof root is `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/`.
- The checked-in generated TS contract now matches the Rust-exported content again, so the focused Rust contract drift guard is green.
- The last focused validation gate that previously blocked this packet now passes in the current checkout:
  - `cmd /c npm run type-check --workspace @ocentra-parent/schema-domain`
- WP06 is therefore complete on the current tree with a real Rust-owned contract, thin/generated TS edge, real tests, a real proof runner, and an honest capability-only/no-daemon boundary.

## Proved states

- capability package only: `service-mode=capability-only`
- launch availability remains explicit:
  - `launch-availability=manual-required`
  - lifecycle phases `simulator-launch=manual-required` and `device-launch=device-proof-required`
- recovery remains explicit:
  - `recovery=not-implemented`
  - lifecycle phase `recovery-behavior=not-implemented`
- provisioning, supervision, signing, TestFlight, background execution, notifications, and physical-device proof remain explicit manual-required, entitlement-required, signing-required, device-proof-required, or planned states
- no hidden daemon or parity claim:
  - `daemon=not-claimed`
  - `child-agent-parity=not-claimed`

## Validation truth

- `rustfmt crates/schema/src/lib.rs crates/schema/src/child_ios_entitlement_capability_proof.rs crates/schema/src/child_ios_entitlement_capability_proof_ts.rs crates/schema/tests/contract.rs crates/schema/tests/contract/child_ios_entitlement_capability_proof.rs`
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `cmd /c npm exec --workspace @ocentra-parent/schema-domain -- vitest run tests/proof/child-ios-entitlement-capability-proof.test.ts`
- `cmd /c npm run test:child-ios-entitlement-capability-proof`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts packages/schema-domain/src/child-ios-entitlement-capability-proof.ts packages/schema-domain/tests/proof/child-ios-entitlement-capability-proof.test.ts scripts/test/child-ios-entitlement-capability-proof.mjs`
- `cargo lint-architecture crates/schema/src/lib.rs crates/schema/src/child_ios_entitlement_capability_proof.rs crates/schema/src/child_ios_entitlement_capability_proof_ts.rs crates/schema/tests/contract.rs crates/schema/tests/contract/child_ios_entitlement_capability_proof.rs`
- `cargo test -p ocentra-schema --test contract child_ios_entitlement_capability` -> pass
- `cmd /c npm run type-check --workspace @ocentra-parent/schema-domain` -> pass

## No-claim boundary

- No persistent iOS background service, daemon, relaunch/recovery service behavior, Family Controls implementation, supervision parity, signing approval, store approval, physical-device install proof, or parent-client parity is claimed from this packet.
