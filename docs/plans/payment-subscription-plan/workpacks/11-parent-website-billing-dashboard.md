# Workpack 11: Parent Website Billing Dashboard

Purpose: define the parent-facing billing dashboard surface and its self-service actions.

## Owns

- `PARENT_WEBSITE_BILLING_DASHBOARD.md`
- the dashboard parts of `RESEARCH_AND_UI_GUIDANCE.md`

## Must prove

- Parent-authenticated users can see their billing state.
- Seat math, invoices, referral credits, and grace state are visible.
- The portal handoff is available.
- Child data and support-only fields stay hidden.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp11/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if the dashboard displays child telemetry.
- The workpack fails if the dashboard lies about entitlement state.
- The workpack fails if the portal handoff is missing.
