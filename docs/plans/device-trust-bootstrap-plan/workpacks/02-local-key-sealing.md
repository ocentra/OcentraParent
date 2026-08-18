# Workpack 02: Local Key Sealing

Purpose: define platform-backed key sealing, fallback behavior, and wrong-device negative cases.

## Owns

- Windows DPAPI behavior.
- Apple Keychain / Secure Enclave behavior.
- Android Keystore behavior.
- Linux keyring adapter boundary.
- Recovery fallback when a platform store is unavailable.
- Local trust-material lifecycle and no-plaintext/no-universal-key boundary.

## Ownership boundary

```text
crates/schema or the owning Rust crate owns canonical key-custody/trust-state shapes. `schema-domain` is temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
device-trust-bootstrap-plan owns the local trust sealing proof contract.
platform-specific runtime owners prove actual platform store behavior when selected.
data-custody-storage-plan owns encrypted recovery artifact custody after trust exists.
account-identity-family-plan owns actor/household authority for recovery and reset.
```

## Required proof fields

The selected proof must name, at minimum:

```text
trust_subject
device_ref
device_role
platform
platform_store
key_owner
sealed_key_state
key_lifecycle_state
rotation_or_revocation_state
recovery_interaction_state
wrong_user_state
wrong_device_state
wrong_key_state
manual_required_state
no_universal_key_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Exit condition

- No plaintext trust keys remain in the model.
- Wrong-user and wrong-device attempts are explicit negative cases.
- Wrong-key and revoked/re-pair states are explicit negative cases where selected.
- The platform store is the security boundary, not the Rust wrapper.
- Recovery interaction is explicit and does not silently restore trust after reinstall.
- Unsupported platforms stay manual-required instead of fake-ready.
- Proof artifacts for this slice live under `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/`.

## Proof target

- `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/`

## Required proof files

```text
output/device-trust-bootstrap-plan-proof/02-local-key-sealing/00-scope-summary.md
output/device-trust-bootstrap-plan-proof/02-local-key-sealing/01-negative-case-proof.md
output/device-trust-bootstrap-plan-proof/02-local-key-sealing/02-no-claim-boundary.md
output/device-trust-bootstrap-plan-proof/02-local-key-sealing/03-platform-proof-status.md
output/device-trust-bootstrap-plan-proof/02-local-key-sealing/16-validation-commands.log
output/device-trust-bootstrap-plan-proof/02-local-key-sealing/17-blockers.md
```

## Companion docs

- `LOCAL_KEY_SEALING_MODEL.md`
- `PLATFORM_KEY_CUSTODY_MATRIX.md`

## Current audit state

The current integration checkout contains Windows custody source, but not a
reachable Windows custody capability. `crates/storage-custody-core/src/
windows_device_trust_custody_platform.rs` defines
`require_authenticated_parent_authority()` as a permanently unavailable
boundary (`Error::Platform`). `WindowsDeviceTrustCustody::seal_persist_activate`,
`unseal_current`, and `revoke_or_reset` all fail at that boundary before they
create or mutate a sealed record, registry epoch, or binding lock. The DPAPI
and current-user registry code is therefore source topology only; it is not a
live sealing, unsealing, activation, or revocation path.

`windows_dpapi_key_sealing.rs` and the opaque
`ParentDeviceTrustCommandFacade` remain bounded source seams. No ceremony
issuer reaches them, no desktop/native command is mounted for the facade, and
no parent-runtime startup composition connects custody to lifecycle activation
or current-binding/revocation state. `device_trust_lifecycle_activation.rs`
defines an opaque authorization consumer, but no production custody caller
supplies it. Caller-provided identifiers, synthetic probes, typed receipts,
or a local staged-handle cache cannot create that missing authority.

The existing Windows custody/lifecycle test files are stale against this
unavailable boundary and are retained only as migration references. They must
not be presented as current DPAPI, lifecycle-activation, desktop-dispatch, or
end-to-end revocation proof. The plan-local route test remains document
alignment coverage only. A future selected platform route must first provide a
real authenticated ceremony issuer, desktop/native mount, startup composition,
and current lifecycle authority before writing fresh platform proof.

## Downstream composition route

WP02 is downstream of the WP01 foundation. When selected for a demonstrated
private-key or install-custody need, its parent-runtime/platform owner must
compose sealing, current-binding lookup, lifecycle generation, and revocation
through the trusted ceremony boundary. WP02 does not issue parent presence,
register a LAN signer, resolve household targets, or revive revoked trust. With
no shipped ceremony issuer or registered native caller, sealing and revocation
remain manual-required; the Windows slice below is source evidence only.

The default Account WP08 -> Cloudflare WP06 -> Device Trust WP03 -> LAN/child
route does not force this workpack. If the platform sealing/lifecycle-revocation
path is selected, the graph route must promote a reviewed `WP26 -> WP02`
`depends_on` edge in `docs/engineering-graph/overrides.json` and the matching
WP26 dependency review before assigning the consumer. The selected edge is
completion-gated (its `implementationGate: "reviewed-implementation"` is only a
source-phase exception), so LAN/child current-binding consumers cannot proceed
until WP02's handoff is complete. Leaving that edge absent is the reviewed
non-sealing route; WP02 is never a ceremony issuer or a reverse dependency on
WP03, so the conditional path remains acyclic.

- The Windows slice is source-only. Because the authenticated-parent authority
  gate is permanently unavailable, the custody methods fail closed before
  record/epoch mutation; no end-to-end seal, unseal, activation, or revocation
  execution is claimed.
- This is not a workpack close. Android, Linux, iOS, and macOS platform custody
  are absent, and no encrypted recovery bundle, re-pair flow, entitlement
  unlock, child removal, or whole device-trust state machine is claimed.
- Revocation remains manual-required for the same reason as sealing. Local
  callers cannot revoke trust by supplying identity strings alone, and no
  current test or synthetic probe supplies DPAPI proof.
- The required proof root is generated locally under
  `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/` and is not
  committed product truth.

## Negative cases

- App preferences cannot store trust keys.
- Reinstall without recovery should not silently restore trust.
- Exporting the key must not be required for normal operation.
- Login/session state cannot unwrap trust material by itself.
- LAN pairing cannot unwrap trust material by itself.
- Package install/copy cannot create a trusted sealed key state by itself.
- Recovery bundle availability cannot bypass wrong-household, wrong-device, or wrong-key rejection.
- A local revoke/reset caller cannot delete sealed trust material without the
  authenticated parent authority; unavailable authority remains manual-required.
