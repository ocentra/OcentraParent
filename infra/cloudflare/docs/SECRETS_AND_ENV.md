# Secrets And Env

Repo rules:

- do not commit `.dev.vars`
- do not commit provider secrets
- do not expose provider or signing secrets to browser, desktop, or mobile clients

Expected secret names:

- `INTERACTIVE_CSRF_TOKEN`
- `STRIPE_SECRET_KEY`
- `STRIPE_WEBHOOK_SECRET`
- `RAZORPAY_KEY_ID`
- `RAZORPAY_KEY_SECRET`
- `PAYPAL_CLIENT_ID`
- `PAYPAL_CLIENT_SECRET`
- `APPLE_STORE_KEY_REF`
- `GOOGLE_PLAY_SERVICE_ACCOUNT_REF`
- `ENTITLEMENT_SIGNING_KEY_REF`
