# Workpack 02: Local Key Sealing

Purpose: define platform-backed key sealing, fallback behavior, and wrong-device negative cases.

## Owns

- Windows DPAPI behavior.
- Apple Keychain / Secure Enclave behavior.
- Android Keystore behavior.
- Linux keyring adapter boundary.
- Recovery fallback when a platform store is unavailable.

## Exit condition

- No plaintext trust keys remain in the model.
- Wrong-user and wrong-device attempts are explicit negative cases.
- The platform store is the security boundary, not the Rust wrapper.
- Proof artifacts for this slice live under `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/`.

## Proof target

- `output/device-trust-bootstrap-plan-proof/02-local-key-sealing/`

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
