# Payment Provider Strategy

Purpose: define which external payment systems are allowed, what role they play, and what they are not allowed to decide.

## Strategy

- Stripe is the primary/default global provider for web and desktop subscriptions, hosted checkout, billing portal, invoices, and webhooks.
- Razorpay is the required India-market adapter for INR and local payment/subscription paths.
- PayPal is the secondary wallet/subscription adapter.
- Apple and Google are store billing adapters only where platform rules require it. Store receipt or token values are provider input only.
- Manual invoice is a support/admin path for schools, enterprise, regional fallback, Pakistan/manual-supported regions, and early pilots.

## Provider authority rule

- Providers can confirm payment, emit webhooks, and hold subscription records.
- Providers do not decide product access.
- The app-owned billing and entitlement ledgers decide access, grace, and revocation.

## Capability matrix

| Provider | Typical use | Authority | Event shape | Notes |
| --- | --- | --- | --- | --- |
| Stripe | Web subscriptions, invoices, portal | Receipt and billing signal only | Checkout, invoice, subscription, entitlement events | Default control-plane path. |
| Razorpay | India billing flows | Receipt and billing signal only | Order, payment, and subscription events | Must normalize to the same ledger model. |
| PayPal | Secondary wallet/subscription flow | Receipt and billing signal only | Order capture and webhook events | Keep it as an adapter, not a product branch. |
| Apple | App Store subscriptions | Store receipt signal only | Receipt and renewal events | Use when store policy requires it. |
| Google | Play Billing subscriptions | Store receipt signal only | Purchase and renewal events | Use when Android store policy requires it. |
| Manual invoice | Support or enterprise billing | App-owned ledger only | Manual invoice and reconciliation entries | No browser-driven payment claim. |

## Rules

- Provider secrets never reach the browser.
- Provider metadata must stay privacy-safe and minimal.
- Provider selection is server-side policy, not a client-side toggle.
- Provider-specific edge cases must normalize to the same entitlement events.
