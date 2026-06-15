# Research and UI Guidance

This document captures the official-source research that informed the trust bootstrap direction and the UI behavior that should follow from it.

## Research findings

| Source | What it supports | Plan implication |
| --- | --- | --- |
| [W3C WebAuthn Level 3](https://www.w3.org/TR/webauthn-3/) | Public-key credentials scoped to a relying party, with `hybrid` transport support. | Desktop QR and phone approval are standards-aligned. |
| [FIDO Passkeys](https://fidoalliance.org/passkeys/) | Cross-device authentication and no shared-secret model. | Prefer passkeys for parent presence proof. |
| [Android Keystore](https://developer.android.com/privacy-and-security/keystore) | Non-exportable key material in platform keystore. | Use platform sealing, not app-managed plaintext keys. |
| [Play Integrity API](https://developer.android.com/google/play/integrity/overview) | Genuine app/device signals for Android. | Treat as supportive evidence only. |
| [Apple Secure Enclave](https://developer.apple.com/documentation/security/protecting-keys-with-the-secure-enclave) | Hardware-based key manager isolated from main processor. | Use Secure Enclave or Keychain classes for local secrets. |
| [Keychain accessibility](https://developer.apple.com/documentation/security/ksecattraccessiblewhenunlockedthisdeviceonly) | This-device-only keychain classes and accessibility control. | Keep trust material nonmigratory when required. |
| [DPAPI](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata) | Encryption bound to the same user and usually the same computer. | Windows trust sealing can be device/user bound. |
| [webauthn-rs](https://github.com/kanidm/webauthn-rs) | Rust relying-party library with a documented security audit. | Candidate for Rust server-side WebAuthn. |
| [passkey-rs](https://github.com/1Password/passkey-rs) | Rust implementation of WebAuthn Level 3 and CTAP2. | Candidate if a Rust client/authenticator boundary is needed. |
| [keyring-rs](https://github.com/open-source-cooperative/keyring-rs) | Cross-platform keyring abstraction. | Candidate adapter for secret-store integration. |
| [rage](https://github.com/str4d/rage) | Rust implementation of the age format. | Candidate for encrypted recovery bundle format. |
| [RustDesk](https://github.com/rustdesk/rustdesk) | Open-source remote desktop stack in Rust with self-hostable infrastructure. | Reference only, not default embedded product code. |

## UI guidance

- On desktop, prefer a QR approval flow when the user is already on a phone-available device.
- On mobile, prefer the OS-native passkey or biometric prompt instead of a custom camera scan or custom face pipeline.
- Show the user whether they are pairing, steping up, approving, recovering, revoking, or re-pairing. Do not hide those transitions behind generic login copy.
- Make trust persistence explicit: once paired, the device stays trusted until parent revocation, removal, or reset.
- Keep child-facing UI honest: do not promise control, self-uninstall, or trust reset from the child device.
- Treat repeated prompts as a failure mode, not a default design pattern.
- Never surface biometric storage language. Surface OS verification language instead.
- Use rescue and recovery language only for encrypted bundle recovery or re-pair, not for casual account recovery.