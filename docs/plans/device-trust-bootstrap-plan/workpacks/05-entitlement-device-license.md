# Workpack 05: Entitlement Device License

Purpose: define signed entitlement snapshots and device-bound license unlock.

## Owns

- Entitlement snapshot fields.
- Signature verification.
- Expiry, grace, and revocation handling.
- Device-bound unlock behavior.

## Exit condition

- Copied binaries or configs do not unlock product behavior.
- The entitlement snapshot is signed and device-bound.
- Revocation overrides stale cache.

## Proof target

- `output/device-trust-bootstrap-plan-proof/05-entitlement-device-license/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Billing entitlement contracts exist elsewhere in the repo, but a device-trust-bound entitlement runtime is still missing.
- Entitlement-core now validates the local account, household, trusted-device,
  package, and active-time bindings before consulting an authority verifier.
  Signature verification and revocation remain unavailable/manual-required;
  the default verifier cannot unlock a capability.

## Historical accepted source checkpoint (pre-candidate wave) — 2026-08-17

The integration branch at `68717b5b7` preserves the Payment-owned unsigned
entitlement projection and a crate-private fail-closed snapshot context. Wire
input discards caller-supplied trust context, and the incompatible public signed
snapshot/verifier modules from the Device branch were removed during review.
No real issuer, signature/revocation provider, or shipped capability-unlock
caller exists. Expected tests, focused execution, and proof remain open.

## Candidate source wave — 2026-08-18 (unreviewed)

The source-only WP05 packet now contains a signed transport envelope and an
authority-owned verification path in `crates/entitlement-core`:

- `entitlement_snapshot_issuer.rs` is crate-internal and accepts only an
  opaque, owner-produced issuance projection. A caller cannot provide the
  signed projection, key identifier, clock values, TTL, or signing receipt.
  The signing provider and its owner composer remain unavailable outside the
  crate; the shipped provider is manual-required.
- `entitlement_snapshot_authority.rs` performs strict Ed25519 verification,
  binds the key identifier to the SHA-256 key identity, resolves package build
  plus bounded release-channel identity from a trusted installed-package
  authority, and resolves billing, grace, family-setup, and policy state from
  a currentness authority. Missing providers fail closed; no positive gate is
  fabricated in the entitlement owner. A live identity re-resolution method
  and owner-held generation fence are required at both grant and consumption.
  Session expiry and the signed active window are evaluated only by the
  crate-private owner-controlled trusted-time/currentness boundary; no caller
  clock is accepted.
  Its raw dependency-injection constructor and typed owner ports are
  crate-private; the only public construction path is the manual-required
  fallback until a concrete owner repository composition is mounted.
- `entitlement_snapshot_cache.rs` persists signed wire data with atomic writes,
  locked replacement, parent-chain symlink/error rejection, timestamp-monotonic
  snapshot replacement, signed revocation generations, and non-shrinking
  revocation membership. Raw transport has no public cache mutation API;
  crate-private replacement requires a verifier receipt, so an unverified
  higher generation cannot poison durable state. The current path checks are
  not a platform handle-safe reparse defense; an owner-controlled custody
  adapter must replace this manual-required path before production reachability.
  Every read remains subject to signature/currentness verification. Snapshot
  authority generation is signed and must exactly match the current revocation
  generation; a separate owner-held monotonic fence is required so rolling
  back both local JSON files cannot restore an old pair.
- `crates/child-runtime/src/runtime_entitlement_license.rs` exposes the real library consumer
  seam `ChildRuntimeEntitlementLicenseStore`; its public startup constructor
  is manual-required only. Its grant is non-cloneable and consumed by value;
  no public grant accessor exposes capability or snapshot identity, and final
  consumption revalidates expiry, revocation generation, installed package,
  account/device binding, live identity generations, and owner-provided gate
  state. No shipped child-agent service startup caller is mounted yet.

Signed grace is explicit but not self-authorizing: after `expires_at`, the
owner-controlled trusted-time/currentness boundary may return `Grace` only when
the signed interval is within that owner's configured maximum grace policy.
There is no crate-wide magic maximum. Missing policy, trusted time, or the
restart-safe rollback fence returns unavailable; the owner must also report
active offline grace, and only the low-risk Tracking capability can pass.
Other capabilities fail closed. Grace is not treated as full entitlement
freshness.

This is a candidate source reachability packet, not an accepted/reviewed
completion claim. A concrete owner repository composition is not present, so
the raw authority/issuer DI path stays crate-private and public startup stays
manual-required. A real issuer/HSM or platform key provider,
installed-package authority, billing/currentness owner, and signed revocation
delivery caller are still external dependencies. No entitlement
activation or broad product capability completion is claimed. The expected
WP05 tests, focused execution, retained proof, CI, and independent review
remain open.

### Required owner route before unlock composition

The next owner packet must be routed through the Account/Billing authority for
the opaque issuance projection and subscription/policy state, the package
release authority for the installed build and release channel, the Device Trust
and Account authorities for live identity/session/generation re-resolution, the
trusted-time/currentness owner for active-window and configured grace policy,
and the revocation owner for signed updates plus a restart-safe generation
fence. A platform custody owner must also replace the path-based mutation with
handle-safe reparse-resistant replacement. The child-agent service owner must
then mount that single composed authority at startup. Until those ownership
edges are present in the graph and backed by real adapters,
`open_manual_required` is the only public startup route and no capability
unlock is available.

## Negative cases

- Wrong device cannot unlock the entitlement.
- Wrong household cannot unlock the entitlement.
- Expired or revoked snapshots fail closed or enter labeled grace only.
