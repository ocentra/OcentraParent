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

## Live validation update (2026-08-09)

The dependency matrix and trust-root boundary test were replayed on the
consolidated E: branch. The durable manifest is
`docs/proof/device-trust-bootstrap-plan/slice-08-dependency-adoption.md`; local
reproducible output is under
`output/device-trust-bootstrap-plan-proof/08-open-source-dependency-adoption/`.

The graph records this docs/review slice as `validation`, not `done`. No
candidate has been wired into the device-trust runtime by this slice, and no
platform ceremony, key sealing, recovery execution, CI, review, or main merge
claim is made.
