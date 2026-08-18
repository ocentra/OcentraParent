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

## Independently reviewed source repairs — 2026-08-18 (expected tests open)

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
  authority, and requires current account/device identity plus trusted-time
  state from the currentness owner. Billing, family-setup, and policy gate
  composition is not present without a concrete action owner. Missing
  providers fail closed; no positive gate is fabricated in the entitlement
  owner. A live identity re-resolution method, trusted monotonic time, bounded
  grace policy, and owner-held generation fence are required before any future
  action owner can authorize or consume.
  Session expiry and the signed active window are evaluated only by the
  crate-private owner-controlled trusted-time/currentness boundary; no caller
  clock is accepted.
  Its raw dependency-injection constructor and typed owner ports are
  crate-private, and no public unlock, capability-selector, or final-consume
  method exists. The verifier is a future owner primitive only; no concrete
  child-runtime action owner currently reaches it.
- `entitlement_snapshot_cache.rs` provides read-only signed revocation state
  with parent-chain symlink/error rejection and shape validation. Snapshot and
  revocation transport have no mutation writer in this packet: the prior
  receipt and raw signed-update paths had no narrowly typed owner transition
  caller and were removed. The current path checks are not a platform
  handle-safe reparse defense; an owner-controlled custody adapter must replace
  this manual-required path before production reachability.
  Every read remains subject to signature/currentness verification. Snapshot
  authority generation is signed and must exactly match the current revocation
  generation; a separate owner-held monotonic fence is required so rolling
  back both local JSON files cannot restore an old pair. Revocation reads are
  signature-checked before they influence currentness.
- No child-runtime entitlement consumer is exported in this packet. The
  previous `runtime_entitlement_license.rs` wrapper and its public
  authorize/consume APIs were removed because no concrete child-runtime action
  owner exists. The child-agent service owner in `service.rs` and
  `service_recovery.rs` does not mount an entitlement authority or provide the
  Account/Device/package/billing/currentness composition.

Signed grace is explicit but not self-authorizing: after `expires_at`, the
owner-controlled trusted-time/currentness boundary may return `Grace` only when
the signed interval is within that owner's configured maximum grace policy.
There is no crate-wide magic maximum. Missing policy, trusted time, or the
restart-safe rollback fence returns unavailable; the owner must also report
active offline grace, and only the low-risk Tracking capability can pass.
Other capabilities fail closed. Grace is not treated as full entitlement
freshness.

This is an independently reviewed source-repair packet, not a completion
claim. A concrete owner repository composition is not present, so
the raw authority/issuer DI path stays crate-private and no entitlement
startup route is exported. A real issuer/HSM or platform key provider,
installed-package authority, billing/currentness owner, and signed revocation
delivery caller are still external dependencies. Expected tests and proof
remain open. No entitlement
activation or broad product capability completion is claimed. The expected
WP05 tests, focused runtime execution, retained proof, and CI remain open.

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
edges are present in the graph and backed by real adapters, no entitlement
startup or capability unlock route is exported. The account/device
generations, NotYetValid, trusted monotonic time, configured grace, and
rollback-fence contracts remain fail-closed owner boundaries; they are not
claimed as concrete runtime enforcement while those owners are absent.

## Negative cases

- Wrong device cannot unlock the entitlement.
- Wrong household cannot unlock the entitlement.
- Expired or revoked snapshots fail closed or enter labeled grace only.
