# Payment Subscription Route Index

Use [AGENTS.md](AGENTS.md), then [PLAN_STATE.md](PLAN_STATE.md), [NEXT_ACTIONS.md](NEXT_ACTIONS.md), and [WORKPACK_INDEX.md](WORKPACK_INDEX.md). Use [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when the selected workpack owner/proof family is unclear.

## Owns

This plan owns billing and monetization semantics:

```text
pricing and seats
checkout and hosted billing portal meaning
provider webhook lifecycle
app-owned billing/referral/entitlement ledgers
signed entitlement snapshot model
invoice/tax/refund/dispute/cancel/grace behavior
provider adapter semantics
regional payment rollout
referral qualification and credit lifecycle
parent billing dashboard content/state
support/admin billing operations
rollout proof and route gate
```

## Boundary split

```text
cloudflare-control-plane-plan owns infra/cloudflare, shared Worker/API module scaffold, auth, bindings, local dev/test, deploy promotion, and payment handoff proof.
billing-domain owns TypeScript billing contract/proof-consumer surfaces when public or selected internal proof surfaces exist.
billing-core owns Rust webhook/lifecycle/eventing helpers when selected.
schema-domain owns canonical shared billing/payment/entitlement shapes when cross-boundary.
account-identity-family-plan owns account, household, role, and session authority.
device-trust-bootstrap-plan owns trusted-device bootstrap and local sealed trust.
data-custody-storage-plan owns privacy, export/delete, and retention boundaries.
setup-install-provisioning-plan owns family-site pricing/register/install entry points.
portal-ux-household-surfaces-plan owns generic household shell UX.
policy-control-plane-plan consumes entitlement state only after account/payment/device-trust authority is proven.
```

## Handoff rule

Open a sibling plan only when the selected workpack names the exact handoff, owner path, expected proof, and no-claim boundary.

## No-claim rule

Do not claim payment runtime readiness from Cloudflare route presence, provider checkout redirects, provider docs, assertion matrices, billing-domain-only tests, parent dashboard projections, or scaffold/spec completeness. Runtime claims require selected proof roots under `output/payment-subscription-plan-proof/<workpack>/`.
