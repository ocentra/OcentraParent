# Workpack 03: Parent Step-Up Auth

Purpose: define parent step-up auth with passkeys, biometrics, and OS-native approval.

## Owns

- High-risk action list.
- Passkey / WebAuthn step-up.
- Native biometric or device-unlock prompts.
- Step-up expiry and replay rules.

## Exit condition

- Each high-risk action has an explicit step-up rule.
- No custom biometric storage appears anywhere in the model.
- Step-up is short-lived, action-bound, and household-bound.

## Proof target

- `output/device-trust-bootstrap-plan-proof/03-parent-step-up-auth/`

## Test layout

- `tests/device-trust-bootstrap-plan/contract/parent-step-up-auth.test.mjs`
- `tests/device-trust-bootstrap-plan/unit/local-key-sealing.test.mjs`
- `tests/device-trust-bootstrap-plan/integration/recovery-re-pair-boundary.test.mjs`
- `crates/family-identity-core/tests/unit/parent_step_up_runtime_fence_participant.rs`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Current plan-local tests for this slice are document assertions; passkey/WebAuthn and OS-native step-up runtime proof are still missing.
- The Rust receipt-shape boundary now rejects receipts whose issued-to-expiry
  lifetime exceeds five minutes before consulting an authority verifier. The
  verifier remains unavailable/manual-required until a real passkey or
  OS-native authority owns signature verification and one-time nonce consume.
- The pushed integration Rust source packet adds atomic challenge/intent issuance,
  atomic receipt/credential consumption, restart reconciliation, and strict
  intent-to-linked-challenge lifecycle validation. Independent static review
  accepted this bounded source shape with no remaining internal P0/P1 in the
  inspected lifecycle paths. The reviewed code map now records it, but no test,
  proof, runtime-reachability, or completion claim follows.
- The draft currently receives child-device and pairing identifiers from the
  request boundary. Account has a sealed binding and local repository, and
  Cloudflare has a read adapter, but WP02 still lacks target-aware action
  authority and WP06 lacks authoritative writes/currentness plus a shipped
  provider caller. Those identifiers therefore cannot authorize signer
  registration.

## LAN WP26 dependency routing

WP03 has three hard prerequisites:

- Device Trust WP01 owns the persistent trusted-device and signer-key lifecycle
  source.
- Account Identity WP08 owns the canonical cross-boundary
  household/child/device/pairing authority contract.
- Cloudflare WP06 owns the durable authoritative repository and production
  caller that persists and resolves that contract after target-aware Account
  WP02. WP02 is transitive through WP06 rather than a duplicate direct edge.

WP03 remains blocked until a real parent ceremony can resolve the target from
those owners, authorize the one-time `RegisterLanSignerAnchor` action, verify
the signed assertion, and consume its nonce/receipt exactly once. The actor
parent-controller device must remain distinct from the target
child/profile/device, and both Account and Device Trust currentness must be
re-resolved. Account WP05
is a pure evaluator over caller-supplied records, and the local LAN trusted
device registry is pairing state; neither is the missing account/household
authority repository. A typed receipt, document assertion, request DTO, or LAN
service caller must not advance WP26.

WP03 is the ceremony owner after the WP01 foundation and Account/Cloudflare
current-authority bridge. WP01 must not become a ceremony issuer, and WP03 must
not depend on LAN WP26 or a child consumer; those consumers are ordered after
the one-time registration authorization and current-binding/revocation handoff.

## Multi-owner effect-fence handoff

WP03 is also the owner of the private parent-step-up reservation participant
consumed by Account WP05A's runtime effect-fencing coordinator. The participant
must reserve and consume the action-bound challenge/receipt, sign-count, expiry,
and target binding in the Device Trust owner; it must not export raw receipt
state or move nonce/replay truth into Account. Account WP05A coordinates this
participant with Account authority, capability, and controller-lease owners
but does not replace it.

The planned `parent_step_up_target_authority.rs`, `parent_step_up_runtime.rs`,
and private participant seams
(`parent_step_up_runtime_fence_participant.rs` in family-identity-core and
parent-runtime-core) remain absent. This handoff adds no source, test, proof,
platform ceremony, or runtime-readiness claim and does not unblock WP03 until
Account WP08/WP02, Cloudflare WP06, Device Trust WP01, and a real passkey/OS
authority are available.

## Implementation-phase routing disposition — 2026-08-17

The graph keeps hard edges from WP03 to Device Trust WP01, Account WP08, and
Cloudflare WP06; target-aware Account WP02 is transitive through WP06. Bounded
ceremony custody source is retained, but the planned production owners
`crates/family-identity-core/src/parent_step_up_target_authority.rs` and
`crates/parent-runtime-core/src/parent_step_up_runtime.rs` are absent. The graph
therefore remains blocked, and normal READY, tests, proof, runtime reachability,
provider/native authority, LAN handoff, and DONE remain unchanged.

## Negative cases

- Cached login cannot bypass step-up.
- Child devices cannot satisfy parent step-up.
- Expired assertions fail closed.
- Parent-controller actor identity cannot be reused as the target child device.
- Cross-child, cross-household, stale Account, revoked Device Trust, provider-
  account mismatch, and replayed `RegisterLanSignerAnchor` attempts fail closed.
