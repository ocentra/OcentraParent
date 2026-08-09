# Device Trust WP08 Dependency Adoption — Validation Manifest

Date: 2026-08-09
Plan: `device-trust-bootstrap-plan`
Workpack: `08-open-source-dependency-adoption`

## Result

The dependency review matrix has explicit adopt-candidate, research-only, and
reference-only decisions for WebAuthn, passkey, keyring, encrypted-bundle,
and RustDesk candidates. The contract test confirms trust-root boundaries and
decision visibility. The graph state is `validation`, not `done`: no dependency
has been wired into a device-trust runtime seam by this slice.

## Commands and results

| Command | Result |
| --- | --- |
| `node --test tests/device-trust-bootstrap-plan/contract/dependency-adoption.test.mjs` | 1 passed, 0 failed |
| `npm run lint:architecture -- --files tests/device-trust-bootstrap-plan/contract/dependency-adoption.test.mjs docs/plans/device-trust-bootstrap-plan/DEPENDENCY_RESEARCH_AND_ADOPTION.md docs/plans/device-trust-bootstrap-plan/workpacks/08-open-source-dependency-adoption.md docs/plans/device-trust-bootstrap-plan/CHECKLIST_INDEX.md` | passed |
| `npm run hub:guard` | passed; no findings/conflicts |

## Decision coverage

- `webauthn-rs`, `keyring-rs`, and `rage`/`age` remain adopt candidates only at
  explicit adapter boundaries.
- `passkey-rs` is research-only until a client/bridge boundary is selected.
- RustDesk is architecture reference only and is not a trust root.
- No dependency is treated as a hidden identity, network, or platform trust
  root.

## No-claim boundary

This manifest does not claim runtime dependency adoption, platform key sealing,
WebAuthn ceremony, recovery bundle execution, device trust, CI, review, or
merge to `main`.
