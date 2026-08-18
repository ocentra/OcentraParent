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
  request boundary. The repository has no durable authoritative
  household-to-child-to-device-to-pairing lookup that can validate those
  identifiers independently, so they cannot authorize signer registration.

## LAN WP26 dependency routing

WP03 has three hard prerequisites:

- Device Trust WP01 owns the persistent trusted-device and signer-key lifecycle
  source.
- Account Identity WP08 owns the canonical cross-boundary
  household/child/device/pairing authority contract.
- Cloudflare WP06 owns the durable authoritative repository and production
  caller that persists and resolves that contract.

WP03 remains blocked until a real parent ceremony can resolve the target from
those owners, authorize the one-time `RegisterLanSignerAnchor` action, verify
the signed assertion, and consume its nonce/receipt exactly once. Account WP05
is a pure evaluator over caller-supplied records, and the local LAN trusted
device registry is pairing state; neither is the missing account/household
authority repository. A typed receipt, document assertion, request DTO, or LAN
service caller must not advance WP26.

WP03 is the ceremony owner after the WP01 foundation and Account/Cloudflare
current-authority bridge. WP01 must not become a ceremony issuer, and WP03 must
not depend on LAN WP26 or a child consumer; those consumers are ordered after
the one-time registration authorization and current-binding/revocation handoff.

## Implementation-phase routing disposition — 2026-08-17

The graph now applies narrow `reviewed-implementation` phase gates from WP03 to
Device Trust WP01, Account WP08, and Cloudflare WP06. WP03's bounded source
packet is authorized for implementation-only continuation against those
reviewed source owners. The default graph remains blocked and normal READY,
tests, proof, runtime reachability, provider/native authority, LAN handoff,
and DONE remain unchanged. No phase gate provides ceremony authority, tests,
proof, completion, or runtime authority, and no WP26 edge is opted into one.

## Negative cases

- Cached login cannot bypass step-up.
- Child devices cannot satisfy parent step-up.
- Expired assertions fail closed.
