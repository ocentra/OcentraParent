# Workpack 06: Security Privacy Observability

Goal: define security and privacy controls for billing.

Expected shape:

- No child names, activity, location, screenshots, policy details, or sensitive evidence in Stripe metadata.
- Logs and metrics redact payment identifiers where appropriate.
- Rate limits, Turnstile or equivalent abuse controls, and webhook DoS handling are required.
- PCI scope stays low by using Stripe-hosted payment UI unless a later decision proves otherwise.

Expected proof:

- Metadata privacy review.
- Secret scan.
- Rate limit/abuse proof.
- Webhook smuggling/desync proof where applicable.
- Alerts and reconciliation reports.

Failure: payment observability leaking family or child safety data.

## Execution Detail

Minimum context:

- `docs/expectations/static-analysis-security.md`
- `docs/expectations/data-custody.md`
- `E:\ocentra-games\infra\cloudflare\src\utils\stripe-webhook-signature.ts`
- `E:\ocentra-games\infra\cloudflare\src\monitoring\security.ts`

Research required:

- Confirm current security expectations for Parent logs and support diagnostics.
- Discuss with Sujan what billing support data Ocentra is allowed to see.
- Confirm provider metadata policy before checkout/webhook implementation.

Forbidden Stripe metadata:

- Child names.
- Child activity.
- Location/geofence data.
- Browser/app/network history.
- Screenshot or screen analysis data.
- Policy details.
- AI safety analysis.

Expected controls:

- Rate limits.
- Bot protection.
- Webhook signature and timestamp tolerance.
- Secret scanning.
- Redacted logs.
- Alerting for webhook failures, payment drift, fraud signals, and repeated checkout abuse.

Expected tests/proof names:

- `payment-security.secret-scan`
- `payment-security.webhook-replay`
- `payment-security.metadata-no-child-data`
- `payment-security.rate-limit`
- `payment-security.redacted-logs`
- `payment-security.alert-fired`

Proof artifact expectations:

- Metadata allow/deny list.
- Log redaction proof.
- Alert/metric examples.
- Abuse test logs.
