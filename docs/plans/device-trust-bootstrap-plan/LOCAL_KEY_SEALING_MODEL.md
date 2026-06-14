# Local Key Sealing Model

This document defines how trust material is sealed on each supported platform.

## Rule

Do not store trust keys in plaintext, in app preferences, or in syncable app storage. Use platform-backed key stores first; if a platform store is unavailable, fall back to encrypted recovery and re-pair, not to an app-managed plaintext secret.

## Platform matrix

| Platform | Preferred sealing target | Notes |
| --- | --- | --- |
| Windows | DPAPI / user or machine bound protection | Bound to the current user and usually the same computer. |
| macOS / iOS | Keychain with this-device-only classes and Secure Enclave where suitable | Keep trust material nonmigratory when required. |
| Android | Android Keystore | Key material remains non-exportable; StrongBox or TEE is a bonus, not the root. |
| Linux | System keyring / Secret Service via keyring abstraction | Good adapter boundary, but not a hardware trust root. |

## Required behavior

- Keys used for trust sealing must not be exportable by normal application code.
- Sealing should be tied to the user/device boundary whenever the platform supports it.
- Reinstall without recovery should not silently restore trust.
- Wrong-user, wrong-device, or revoked-device attempts must fail.
- Logs must never print trust material.

## Negative cases

- A copied trust file cannot decrypt on a different user account or device when the platform binding is working.
- A corrupted store entry must fail closed, not silently create a new trust root.
- No trust secret should live in browser storage, local storage, or plain JSON config.

## Adoption note

`keyring-rs` is a candidate abstraction for platform storage integration, but the plan still treats the platform store as the security boundary, not the Rust adapter.