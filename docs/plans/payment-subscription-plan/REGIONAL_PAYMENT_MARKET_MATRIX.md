# Regional Payment Market Matrix

Purpose: define the default provider and fallback policy by market before implementation claims.

| Region / surface             | Default provider    | Fallback                               | Notes                                             | Proof required                                              |
| ---------------------------- | ------------------- | -------------------------------------- | ------------------------------------------------- | ----------------------------------------------------------- |
| United States / Canada       | Stripe              | PayPal or manual invoice               | Web-first launch path.                            | Provider availability, checkout, portal, and webhook proof. |
| Europe / UK                  | Stripe              | PayPal or manual invoice               | Keep tax and invoice handling explicit.           | Tax, invoice, and refund proof.                             |
| India                        | Razorpay            | Stripe if enabled, then manual invoice | India-native payment methods should be available. | Regional adapter and webhook proof.                         |
| Rest of world                | Stripe if available | PayPal or manual invoice               | Market selection must stay server-side.           | Regional fallback and ledger proof.                         |
| Apple App Store distribution | Apple IAP           | Manual support path                    | Only when store policy requires it.               | Store receipt and entitlement proof.                        |
| Google Play distribution     | Google Play Billing | Manual support path                    | Only when store policy requires it.               | Store receipt and entitlement proof.                        |

## Rollout rules

- Regional rollout is a product decision, not a provider accident.
- The matrix must be closed before a region is marked live.
- A missing provider must degrade to a supported fallback, not a silent failure.
- Tax, currency, and invoice behavior must be checked per region before launch.
