# Workpack 08: Open Source Dependency Adoption

Purpose: evaluate WebAuthn, keyring, and encrypted-bundle dependencies for adoption.

## Owns

- Use case fit.
- License fit.
- Maintenance fit.
- Security review fit.
- Unsafe / FFI / network behavior review.
- Platform coverage review.
- Adapter boundary and replaceability.

## Exit condition

- Every candidate is marked adopt, research-only, or reject.
- No dependency is treated as a hidden trust root.
- Security, license, and maintenance tradeoffs are explicit.

## Proof target

- `docs/proof/device-trust-bootstrap-plan/08-*`

## Negative cases

- Rust does not automatically mean safe to adopt.
- AGPL or large remote-desktop code does not become the trust root.
- A keyring adapter does not replace the platform store boundary.