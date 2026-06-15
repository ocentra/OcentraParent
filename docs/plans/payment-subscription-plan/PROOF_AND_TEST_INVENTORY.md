# Proof and Test Inventory

Purpose: define the proof expected for each workpack. This document is a map, not the proof itself.

## Storage rule

- Proof artifacts live in the designated local artifact path outside this plan folder, typically `docs/proof/payment-subscription-plan/` during local work or the owning crate's local proof directory.
- Keep proof artifacts uncommitted until PR time unless the workpack explicitly says otherwise.
- Each proof file must name the workpack, slice, provider or region if relevant, commands run, positive and negative cases, artifacts, and manual-required notes.

## Global proof categories

- Architecture and route sync proof.
- Unit tests for formulas, ledger transitions, and normalization.
- Integration tests for webhook replay, checkout completion, portal sessions, and provider adapters.
- Negative tests for missing secrets, invalid signatures, unauthenticated access, and unsupported regions.
- Security proof for redaction, metadata minimization, and test/live isolation.
- Ops proof for retry, dead-letter, reconciliation, and manual adjustment flows.

## Workpack proof matrix

| Workpack                            | Minimum proof expectations                                                                                                                 |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| WP01 pricing and entitlement        | Seat formula tests, referral credit math, starter bundle tests, overflow/negative input rejection, and no-child-data proof.                |
| WP02 checkout and portal            | Checkout session creation, portal session creation, missing-secret rejection, unauthenticated rejection, and redirect-not-proof rejection. |
| WP03 webhook lifecycle              | Signature verification, duplicate event rejection, out-of-order event handling, retry handling, and idempotency proof.                     |
| WP04 entitlement gates              | Signed snapshot creation, device binding rejection, stale snapshot rejection, revocation propagation, and no-child-data proof.             |
| WP05 invoice/tax/refund/dispute     | Invoice finalization, tax line recording, full and partial refunds, dispute open/close, cancellation, and grace transitions.               |
| WP06 security/privacy/observability | Secret redaction, provider metadata minimization, log redaction, audit trail, rate limiting or abuse gating, and test/live separation.     |
| WP07 rollout proof and route gate   | Route/doc sync, proof pointer outside the plan folder, validation command log, and PR-ready gate closure.                                  |
| WP08 provider portability           | Normalized success/failure across Stripe, Razorpay, PayPal, store, and manual invoice adapters.                                            |
| WP09 regional rollout               | Region matrix selection, fallback routing, currency/tax handling, and region-disabled failure behavior.                                    |
| WP10 referral growth                | Referral qualification, self-referral rejection, duplicate rejection, revoke/grace behavior, and anti-abuse audit proof.                   |
| WP11 parent dashboard               | Parent authZ, redacted billing view, seat and invoice display, portal handoff, and no-child-data proof.                                    |
| WP12 support/admin ops              | Admin authZ, redacted account search, manual adjustment audit, refund/dispute actions, and safe replay handling.                           |

## Expected command families

- Docs-only slices: format and architecture lint for the touched files.
- Contract or schema slices: build, schema lint, and contract tests for the touched boundary.
- Cloudflare implementation slices: worker tests, wrangler validation, and the touched package or crate's full validation command.
- Security or ops slices: add redaction, replay, and rollback or teardown proof, not just happy-path tests.
