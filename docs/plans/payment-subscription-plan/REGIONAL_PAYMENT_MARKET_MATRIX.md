# Regional Payment Market Matrix

Purpose: define the default provider and fallback policy by market before implementation claims.

| Region / surface | Default provider | Fallback | Notes | Proof required |
| --- | --- | --- | --- | --- |
| Canada/US | Stripe | PayPal or manual invoice | Default web/desktop launch path. | Provider availability, checkout, portal, and webhook proof. |
| India | Razorpay | Stripe if enabled, then manual invoice | India-native payment methods should be available. | Regional adapter and webhook proof. |
| Pakistan | Manual invoice | Support/admin fallback | Provider choice is manual-required until market support is approved. | Manual invoice and entitlement proof. |
| China | Manual/enterprise | Support/admin fallback | Market/provider choice is manual-required until legal and provider support are confirmed. | Manual/enterprise routing proof. |
| UAE/Dubai | Stripe if available | Manual invoice | Merchant/tax setup is manual-required until launch is approved. | Region and tax routing proof. |
| EU/UK | Stripe | PayPal or manual invoice | Keep tax and invoice handling explicit. | Tax, invoice, and refund proof. |
| Southeast Asia | Stripe if available | PayPal or manual invoice | Market selection must stay server-side. | Regional fallback and ledger proof. |
| Manual/enterprise | Manual invoice | Support/admin fallback | Support and enterprise workflows remain supported. | Manual invoice and support proof. |

## Rollout rules

- Regional rollout is a product decision, not a provider accident.
- The matrix must be closed before a region is marked live.
- A missing provider must degrade to a supported fallback, not a silent failure.
- Tax, currency, and invoice behavior must be checked per region before launch.
