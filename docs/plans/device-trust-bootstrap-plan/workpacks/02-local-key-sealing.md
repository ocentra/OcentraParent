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

## Proof target

- `docs/proof/device-trust-bootstrap-plan/02-*`

## Negative cases

- App preferences cannot store trust keys.
- Reinstall without recovery should not silently restore trust.
- Exporting the key must not be required for normal operation.