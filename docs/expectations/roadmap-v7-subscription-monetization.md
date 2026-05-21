# V7 Subscription And Monetization Expectations

This is the milestone-specific expectation file for V7 in `docs/product-roadmap.md`.

Supporting expectation files: [billing](billing.md), [portal](portal.md), [cloud](cloud.md), and [static analysis and security](static-analysis-security.md).

## Outcome

- Plans, trials, Stripe billing, device limits, subscription sync, admin, and support flows are sellable without entering child-device safety internals.
- Child-device agents consume only Ocentra-owned typed entitlement snapshots or local grace state.
- Billing failures are visible and auditable.

## Acceptance

- Paid features are entitlement-gated through typed contracts.
- Billing provider references and secrets stay behind backend boundaries.
- Local safety behavior degrades deliberately and visibly when billing checks are unavailable, expired, or disputed.

## Validation

- Run `npm run validate`.
- Include billing contract tests, backend boundary tests, secret scans, failure/grace tests, and portal entitlement-state coverage.
