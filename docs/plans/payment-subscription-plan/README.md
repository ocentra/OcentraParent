# Payment Subscription Plan

This plan owns parent-product monetization: Cloudflare billing control plane, subscriptions, referral credits, household/device entitlements, checkout, billing portal, webhooks, refunds, disputes, invoices, taxes, privacy-safe metadata, dashboards, and paid-feature gating.

It reuses the Cloudflare-style control-plane shape summarized in the route docs, but it is not a copy of game payments. Games payments include credits, marketplace-style providers, and game-economy flows. Parent payments are family subscriptions and must avoid leaking child data to payment providers.

Start with [AGENTS.md](AGENTS.md).
