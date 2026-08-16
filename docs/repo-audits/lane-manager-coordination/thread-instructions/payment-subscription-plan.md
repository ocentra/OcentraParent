# payment-subscription-plan Instruction

## Verdict

`partial / blocked by Cloudflare handoff`. Proof routing and local crate fixes are useful, but no payment proof bundles exist yet.

## Assign first

Only after Cloudflare has started CFCP-C or published handoff progress, assign `payment-worker-and-domain-proof-alpha`:

- prove `packages/billing-domain` pricing, entitlement, checkout, refund/dispute, redaction surfaces;
- produce `output/payment-subscription-plan-proof/*` artifacts;
- keep Cloudflare worker/control-plane source owned by `cloudflare-control-plane-plan`.

## Then

1. `payment-parent-surface-proof`.
2. `payment-core-crate-proof`.
3. `payment-cloudflare-handoff-consume` after `payment-handoff-proof.md` exists.
4. provider/region/store proof.

## Coordinate with

- `cloudflare-control-plane-plan` for WP00/WP12 handoff.
- `account-identity-family-plan` and `device-trust-bootstrap-plan` for final entitlement device/household subject semantics.

## Do not

- Do not infer payment readiness from Cloudflare source/tests alone.
- Do not claim provider/store/region readiness without proof bundles.
- Do not edit Cloudflare shared runtime from payment lane unless lane manager grants ownership.
