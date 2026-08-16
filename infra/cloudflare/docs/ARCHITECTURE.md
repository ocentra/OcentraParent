# Parent Cloudflare Architecture

This module is the shared Cloudflare control-plane runtime for Parent.

Current state:

- repo-local module exists
- safe worker entrypoint exists
- route manifest exists
- real billing/control-plane handlers exist
- real local binding-backed read model and fixtures exist
- real unit, integration, e2e, contract, security, property, and fuzz suites exist
- auth provider authority, deploy proof, and payment handoff proof remain open
- several planned subdirectories are still scaffold-only `README.md` surfaces while runtime stays concentrated in `src/index.ts`, `src/billing-binding-read-model.ts`, and `src/fixtures.ts`

Ownership:

- shared module shape: `docs/plans/cloudflare-control-plane-plan/`
- payment semantics: `docs/plans/payment-subscription-plan/`
- auth provider decision: `docs/plans/account-identity-family-plan/`
- trusted-device gate consumption: `docs/plans/device-trust-bootstrap-plan/`
