# Workpack 01: Product Pricing Entitlement

Goal: define parent-product pricing and entitlement without importing game economy assumptions.

Expected shape:

- Household subscription tiers.
- Optional trial and grace-period semantics.
- Device-seat or child-profile limits if used.
- Paid feature gates are explicit and parent-visible.
- Free/safety-critical boundaries are explicit so payment failure does not create unsafe hidden behavior.

Expected proof:

- Product/price fixture matrix.
- Upgrade/downgrade/cancel/expired trial proof.
- Entitlement boundary proof.
- Account/household handoff proof.

Failure: treating game credits, marketplace, or tournament payments as the default parent-product model.

## Execution Detail

Minimum context:

- `docs/expectations/billing.md`
- `docs/roadmaps/roadmap-v7-subscription-monetization.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`
- `E:\ocentra-games\infra\cloudflare\src\config\products.ts`
- `E:\ocentra-games\infra\cloudflare\src\config\plan-tiers.ts`

Research required:

- Compare games product catalog concepts to Parent needs.
- Discuss pricing model with Sujan before locking tiers.
- Verify whether MVP is free, trial, subscription, family seat, device seat, or paid support.

Required decisions:

- Billing unit: household, parent account, child profile, child device, remote-access add-on, or support tier.
- Trial/grace period behavior.
- What features remain available when payment fails for child safety.
- Entitlement source of truth and sync to offline devices.

Expected tests/proof names:

- `billing-pricing.tier-matrix`
- `billing-pricing.trial-grace-boundary`
- `billing-pricing.safety-critical-free-boundary`
- `billing-pricing.entitlement-source-owner`

Proof artifact expectations:

- Pricing decision record.
- Feature entitlement matrix.
- Rejected game-economy assumptions.
- Sujan decision note for unresolved product pricing.
