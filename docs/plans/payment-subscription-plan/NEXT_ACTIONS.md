# Next Actions

## Scope and ownership

- Plan owner: `docs/plans/payment-subscription-plan/AGENTS.md`, plus the billing contracts and dashboard lanes.
- Ownership domain: pricing units, referral credits, subscription state, entitlement math, provider adapters, and billing-state transitions.
- Scope boundary: billing/entitlement proof, Worker/API contracts, and privacy-safe parent metadata only.

## Decision routes and failure conditions

- If provider matrix or regional rollout is incomplete, stay on the research lane.
- If tier, grace, or cancellation semantics are undefined, block backend implementation.
- If privacy-safe payload rules are not defined, pause child-facing or support-facing UI work.
- If proof artifacts live inside the plan folder, move them out before claiming progress.

## Actioned completion tracker

- [ ] Lock PSP-001 through PSP-012 with Sujan.
- [ ] Define starter bundle, paid child-seat pricing, and referral credit math.
- [ ] Define Worker/API checkout, portal, and webhook boundaries.
- [ ] Define subscription lifecycle events and entitlement state transitions.
- [ ] Define privacy-safe metadata and forbidden provider fields.
- [ ] Define invoice, tax, refund, dispute, cancellation, and grace semantics.
- [ ] Define the proof matrix, route sync, and external artifact path.
