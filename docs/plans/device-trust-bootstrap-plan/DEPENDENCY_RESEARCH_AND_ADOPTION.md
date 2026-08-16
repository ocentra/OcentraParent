# Dependency Research and Adoption

This document records the external-library review for the device-trust bootstrap plan.

## Review matrix

| Dependency | Use case | License | Maintenance / review | Unsafe / FFI | Network behavior | Platform coverage | Adapter boundary | Replaceability | Decision |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `webauthn-rs` | Rust relying-party WebAuthn server support. | MPL-2.0. | Active, with a documented security audit. | Safe Rust API. | No network root by itself. | Browser-facing RP side. | Server-side auth boundary. | Replaceable if a different RP library proves better. | Adopt candidate. |
| `passkey-rs` | Rust WebAuthn Level 3 / CTAP2 client or authenticator flow support. | Apache-2.0 / MIT. | Active Rust implementation. | Mostly safe Rust. | No network root by itself. | Good for passkey ceremony work. | Optional client / bridge boundary. | Replaceable if native browser or OS APIs suffice. | Research-only until a client boundary is chosen. |
| `keyring-rs` | Cross-platform secret-store adapter. | Apache-2.0 / MIT. | Active, but the crate itself says app code should depend on `keyring-core`. | Safe Rust adapter with platform backends. | No network root by itself. | Windows, macOS, iOS, Linux, Android, and more via backends. | Adapter over platform secret stores. | Replaceable by a different adapter. | Adopt candidate as an adapter, not as trust root. |
| `rage` / `age` format | Encrypted recovery bundle and backup format. | Apache-2.0 / MIT for `rage`; age format reference is open and widely implemented. | Active Rust implementation and active format ecosystem. | Safe Rust library / toolchain. | No network root by itself. | Portable across supported hosts. | Recovery bundle encryption boundary. | Replaceable if a better portable format is proven. | Adopt candidate for encrypted bundles. |
| `RustDesk` | Remote-desktop architecture reference. | AGPL-3.0. | Active project, but full stack is large. | Large Rust and native surface. | Includes relay / rendezvous network behavior. | Cross-platform remote desktop. | Reference only, not embedded trust root. | Replaceable by owned transport and view stack. | Research-only reference. |

## Review criteria

For every candidate, record:

- Use case.
- License.
- Maintenance status.
- Security review status.
- Unsafe or FFI risk.
- Network behavior.
- Platform coverage.
- Adapter boundary.
- Replaceability.
- Adopt / research-only / reject.

## Negative cases

- Do not adopt a dependency just because it exists in Rust.
- Do not let AGPL or large remote-desktop code become the hidden product trust root.
- Do not let a key-store adapter replace the platform security boundary.