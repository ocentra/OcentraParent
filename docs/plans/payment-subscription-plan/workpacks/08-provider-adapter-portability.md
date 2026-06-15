# Workpack 08: Provider Adapter Portability

Purpose: define the normalized adapter contract across Stripe, Razorpay, PayPal, store billing, and manual invoice.

## Owns

- `PAYMENT_PROVIDER_STRATEGY.md`
- `MOBILE_STORE_BILLING_ADAPTERS.md`
- PSP-005

## Must prove

- Provider-specific events normalize to one app-owned ledger model.
- Provider config missing cases fail safely.
- One provider adapter does not leak into another.
- Manual invoice stays a supported adapter, not a separate billing authority.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp08/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if provider-specific code paths own product access.
- The workpack fails if missing config produces a silent fallback.
- The workpack fails if adapter outputs do not normalize.
