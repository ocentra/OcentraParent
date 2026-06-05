SOCIAL-13 source snapshot

Implemented source:

- `packages/activity-domain/src/browser-social-account-creation-gate.ts`
- `packages/activity-domain/tests/browser-social-account-creation-gate.test.ts`
- `scripts/test/social-account-creation-live-proof.mjs`
- `test-results/social-account-creation-live-proof/proof.json`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/11-live-proof.json`
- `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/06-live-screenshots/`

Scope:

- Defines a managed-browser social account gate-plan contract.
- Requires matching route-only account-flow evidence and sanitized form-shape evidence.
- Carries policy decision candidate refs and parent approval request refs as strings without importing parent-domain.
- Models allow-navigation, hold-for-parent-approval, block-submit, manual-review, and unknown-flow warning candidates.
- Captures real public browser surfaces with Playwright for Facebook signup, Pinterest login, Reddit register, and Instagram signup.
- Extracts visible sanitized control kinds only, then builds schema-validated gate plans from the existing route, account-flow, form-shape, and gate contracts.

Pending:

- Runtime browser pause/block, child/parent UI, final policy execution, native app support, connector authorization, enforcement, and product checklist claims are not implemented.
