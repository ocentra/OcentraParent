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
schema-domain owns canonical key-custody/trust-state shapes.
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

- No proof root currently exists on disk for this workpack.
- Platform custody expectations are modeled, but platform-backed sealing runtime proof is still missing on Windows, Android, and Linux; iOS and macOS proof remain external-platform constraints from this host.

## Negative cases

- App preferences cannot store trust keys.
- Reinstall without recovery should not silently restore trust.
- Exporting the key must not be required for normal operation.
- Login/session state cannot unwrap trust material by itself.
- LAN pairing cannot unwrap trust material by itself.
- Package install/copy cannot create a trusted sealed key state by itself.
- Recovery bundle availability cannot bypass wrong-household, wrong-device, or wrong-key rejection.
