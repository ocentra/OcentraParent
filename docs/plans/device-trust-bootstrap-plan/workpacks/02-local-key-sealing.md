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

- The merged Windows-only custody and dispatch safety slice has an unregistered
  native parent-runtime facade consumes only an opaque staged-ceremony handle and
  rejects an unstaged or already-consumed handle before it can call
  `storage-custody-core`. No parent desktop bridge action is registered for this
  facade, so this adapter-local rejection does not prove a live desktop-command or
  webview dispatch path. There is no record-backed household-authority owner yet, so
  no ceremony issuer is exported to external/runtime callers; caller-supplied
  authority flags cannot mint a ceremony. That adapter
  source DPAPI-protects locally generated trust material with its family/account/device
  binding, atomically persists the ciphertext record, and then activates a
  separately DPAPI-protected epoch below the current user's Windows registry
  hive. Revocation removes that epoch before best-effort record deletion, and
  without an operational issuer, no end-to-end Windows seal or restored-record
  execution proof is claimed. The focused custody test covers only idempotent
  revocation of an unissued binding. Production parent-presence custody remains
  fail-closed and does not yet stage operational ceremonies.
- This is a merged Windows-only source-and-test slice, not a workpack close. Android,
  Linux, iOS, and macOS platform custody are still absent. No encrypted recovery
  bundle, re-pair flow, entitlement unlock, child removal, or whole
  device-trust state machine is claimed here.
- The production revocation path now applies the same authenticated-parent
  authority gate as sealing. Without an owning ceremony/authority provider it
  remains manual-required and preserves the sealed record and platform epoch;
  local callers cannot revoke trust by supplying identity strings alone.
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
