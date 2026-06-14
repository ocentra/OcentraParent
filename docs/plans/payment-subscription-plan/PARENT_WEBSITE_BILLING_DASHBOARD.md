# Parent Website Billing Dashboard

Purpose: define the parent-facing surface that shows billing state and self-service actions.

## Required sections

- Current plan and starter bundle summary.
- Active child-device seat count.
- Referral credit count and referral status.
- Next renewal, grace state, and payment status.
- Invoice history and provider mode.
- Self-service entry to the hosted billing portal.

## Rules

- The dashboard must be readable on web and mobile.
- The dashboard must not show child telemetry, child screenshots, or policy details.
- The dashboard may show support-safe account and household identifiers.
- The dashboard must reflect the app-owned ledger, not the provider UI alone.

## Required behaviors

| Action                  | Expected result                                       |
| ----------------------- | ----------------------------------------------------- |
| View status             | Show the current billing and entitlement state.       |
| Open portal             | Hand off to the hosted portal for payment actions.    |
| Review invoices         | Show invoice history and amounts.                     |
| Review referral credits | Show whether credits are active, pending, or revoked. |

## Failure conditions

- Do not expose support-only fields.
- Do not expose child data.
- Do not let the dashboard claim entitlement before the ledger does.
