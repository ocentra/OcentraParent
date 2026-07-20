<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: `Payment Subscription Workpack Index`
> Kind: workpack selector.
> Read when: after PLAN_STATE.md and NEXT_ACTIONS.md.
> Stop rule: open exactly one selected workpack; do not read every workpack.
> Proves: workpack routing only.
> Does not prove: payment runtime readiness, entitlement readiness, regional readiness, or PR readiness.
> Proof rule: update counts/status only after matching proof artifacts exist.

<!-- /agent-capsule -->

# Payment Subscription Workpack Index

Status column reflects runtime execution state, not engineering-spec completeness.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Proof root |
| --- | --- | ---: | --- |
| blocked / proof-required | [WP00 Cloudflare Control Plane Handoff](workpacks/00-cloudflare-control-plane-handoff.md) | 8/9 | `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/` |
| done / proof-required | [WP01 Product Pricing Entitlement](workpacks/01-product-pricing-entitlement.md) | 12/12 | `output/payment-subscription-plan-proof/01-product-pricing-entitlement/` |
| blocked / proof-required | [WP02 Checkout Billing Portal](workpacks/02-checkout-billing-portal.md) | 12/12 | `output/payment-subscription-plan-proof/02-checkout-billing-portal/` |
| blocked / proof-required / WP00-blocked | [WP03 Subscription Webhook Lifecycle](workpacks/03-subscription-webhook-lifecycle.md) | 12/12 source; 0/1 proof custody | `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/` |
| blocked / proof-required / WP00-blocked | [WP04 Entitlement Delivery Gates](workpacks/04-entitlement-delivery-gates.md) | 12/12 source; 0/1 proof custody | `output/payment-subscription-plan-proof/04-entitlement-delivery-gates/` |
| open | [WP05 Invoice Tax Refund Dispute](workpacks/05-invoice-tax-refund-dispute.md) | 0/12 | `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/` |
| open | [WP06 Security Privacy Observability](workpacks/06-security-privacy-observability.md) | 0/12 | `output/payment-subscription-plan-proof/06-security-privacy-observability/` |
| open | [WP08 Provider Adapter Portability](workpacks/08-provider-adapter-portability.md) | 0/12 | `output/payment-subscription-plan-proof/08-provider-adapter-portability/` |
| open | [WP09 Regional Payment Rollout](workpacks/09-regional-payment-rollout.md) | 0/12 | `output/payment-subscription-plan-proof/09-regional-payment-rollout/` |
| open | [WP10 Referral Growth Entitlement](workpacks/10-referral-growth-entitlement.md) | 0/12 | `output/payment-subscription-plan-proof/10-referral-growth-entitlement/` |
| open | [WP11 Parent Website Billing Dashboard](workpacks/11-parent-website-billing-dashboard.md) | 0/12 | `output/payment-subscription-plan-proof/11-parent-website-billing-dashboard/` |
| open | [WP12 Support Admin Billing Ops](workpacks/12-support-admin-billing-ops.md) | 0/12 | `output/payment-subscription-plan-proof/12-support-admin-billing-ops/` |
| open | [WP07 Rollout Proof And Route Gate](workpacks/07-rollout-proof-and-route-gate.md) | 0/14 | `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/` |

For WP03 and WP04, `12/12 source` preserves the independently inspectable implementation/test checklist evidence; `0/1 proof custody` records that the canonical physical bundle is absent. Neither row is complete while WP00 is unaccepted or proof custody is zero.

## Default execution order

```text
WP00 -> WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06 -> WP08 -> WP09 -> WP10 -> WP11 -> WP12 -> WP07
```

## Dependency rules

```text
WP00 blocks payment runtime work.
WP01 defines product/pricing/seat/referral math before checkout/entitlement code.
WP02 depends on WP00/WP01.
WP03 depends on provider boundary from WP02 and owns webhook lifecycle.
WP04 depends on WP01/WP03 and device-trust/account handoffs.
WP05 depends on WP03 lifecycle and regional/provider rules.
WP06 can run in parallel but cannot close rollout without runtime proof.
WP08/WP09 define provider portability and regional rollout before production claims.
WP10 depends on WP01/WP04 credit/entitlement model.
WP11 depends on account/setup/cloudflare handoffs.
WP12 depends on WP03/WP05 and admin/support boundaries.
WP07 is last and consumes all previous proof roots.
```

## Selection rules

- Choose exactly one workpack.
- If the selected workpack owner/proof family is unclear, classify it through `WORKPACK_FAMILIES.md`; do not scan every family.
- Do not start any runtime workpack while WP00 lacks accepted Cloudflare handoff proof or an exact blocker.
- Do not use Cloudflare scaffold, route presence, provider docs, assertion matrix rows, or spec completeness as runtime proof.
- Do not use billing-domain tests as parent dashboard proof without the targeted parent-surface proof.
- Do not use provider checkout/portal proof as entitlement proof.

## Do not select

Do not implement account identity, device trust, Cloudflare scaffold, data custody, setup/install, policy semantics, enforcement behavior, or child telemetry in this plan.
