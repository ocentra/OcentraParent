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

- `output/device-trust-bootstrap-plan-proof/08-open-source-dependency-adoption/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- The dependency matrix lives in `DEPENDENCY_RESEARCH_AND_ADOPTION.md`, but adoption remains research-level until chosen adapters or libraries are wired into owned runtime seams with scoped proof.

## Negative cases

- Rust does not automatically mean safe to adopt.
- AGPL or large remote-desktop code does not become the trust root.
- A keyring adapter does not replace the platform store boundary.
