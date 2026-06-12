# Next Actions

## Scope and ownership

- Plan owner: `docs/plans/payment-subscription-plan/AGENTS.md`, plus billing contracts and portal UX lanes.
- Ownership domain: plan lifecycle data model, subscription contract, entitlement math, and customer billing state transitions.
- Scope boundary: billing/entitlement proof, worker/API boundary contracts, and non-activity parent metadata only.

## Decision routes and failure conditions

- Decision path:
  - If provider contract is incomplete -> remain on research lane and do not create implementation artifacts.
  - If plan tier/grace/cancel states are undefined -> add a blocking `policy-risk` step before backend implementation.
  - If privacy-safe payload rules are not defined -> pause child-facing UX work.
- Failure modes:
  - Missing entitlement lifecycle proof.
  - Undefined retry/grace/cancellation matrix.
  - Undefined privacy boundary for payment metadata.

## Actioned completion tracker

- [ ] Define household pricing, trials, plan tiers, and device-seat semantics.
- [ ] Define Worker/API checkout and customer-portal boundary.
- [ ] Define subscription lifecycle events and entitlement state.
- [ ] Define privacy-safe metadata and forbidden Stripe fields.
- [ ] Define tax/invoice/refund/dispute/cancellation/grace-period states.
- [ ] Define security/proof matrix and route/index sync.
