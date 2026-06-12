<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: README
> Kind: token-efficient entry point.
> Read when: billing, subscriptions, payment, pricing, entitlement, refund, invoice, or Stripe work is in scope.
> Stop rule: Open AGENTS, then one workpack.
> Proves: route ownership only.

<!-- /agent-capsule -->

# Payment Subscription Plan

This plan owns parent-product monetization: subscription plans, household/device entitlements, checkout, billing portal, webhooks, refunds, disputes, invoices, taxes, privacy-safe metadata, and paid-feature gating.

It is informed by `E:\ocentra-games`, but it is not a copy of game payments. Games payments include credits, marketplace-style providers, and game economy flows. Parent payments are family subscriptions and must avoid leaking child data to payment providers.

Start with [AGENTS.md](AGENTS.md).
