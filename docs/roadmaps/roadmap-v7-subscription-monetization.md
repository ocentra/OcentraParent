<!-- agent-capsule -->

> Agent Capsule
> Doc: V7 Subscription And Monetization Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V7 Subscription And Monetization Expectations

This is the milestone-specific expectation file for V7 in `docs/product-roadmap.md`.

Supporting expectation files: [billing](../expectations/billing.md), [portal](../expectations/portal.md), [cloud](../expectations/cloud.md), and [static analysis and security](../expectations/static-analysis-security.md).

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
