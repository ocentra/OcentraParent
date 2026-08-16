# Platform Key Custody Matrix

## Purpose

Define the custody boundary for trust keys sealed locally. The platform store is the security boundary; the Rust wrapper is only an adapter.

## Custody rules

- trust keys remain inside platform-backed stores or sealed recovery artifacts
- normal application code must not export or serialize trust keys
- if the platform store is unavailable, fallback is encrypted recovery and re-pair
- corrupted or mismatched entries fail closed
- copied trust material must not become valid on another user or device when platform binding is working

## Matrix

| Platform | Preferred custody target | Binding expectation | Failure condition | Notes |
| --- | --- | --- | --- | --- |
| Windows | DPAPI user- or machine-bound protection | current user or current machine, depending on the selected mode | copy to another user or device cannot unlock | user-bound is the stronger default |
| macOS | Keychain with this-device-only classes; Secure Enclave where suitable | current device | migrate, export, or restore to another device cannot unlock | nonmigratory classes keep trust local |
| iOS | Keychain with this-device-only classes; Secure Enclave where suitable | current device | device restore or export cannot unlock | biometric prompts stay inside the OS |
| Android | Android Keystore | current device and app identity | export or plain-text persistence cannot unlock | StrongBox or TEE is a bonus, not the root |
| Linux | System keyring / Secret Service via a keyring abstraction | current user session and keyring service | plaintext config or portable copy cannot unlock | adapter boundary only; not a hardware trust root |

## Failure modes

- missing platform store -> encrypted recovery and re-pair
- corrupted store -> fail closed
- normal app export path -> denied
- app prefs, sync, and local storage -> not valid custody targets

## Boundary note

`keyring-rs` may be a candidate adapter, but it is not the custody boundary. The platform store remains the trust root.
