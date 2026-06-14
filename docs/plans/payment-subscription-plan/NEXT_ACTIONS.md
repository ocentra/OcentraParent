# Next Actions

## Current slice

- Current slice: `01. Cloudflare billing control plane`
- Current owner: `payment-subscription-plan`
- Current status: `pending`

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
|---|---|---|---|---|---|
| 01 | Cloudflare billing control plane | pending | `packages/billing-domain/src/billing-account-runtime-boundary.ts` | Read `AGENTS.md`, `PLAN_STATE.md`, `CLOUDFLARE_BILLING_CONTROL_PLANE.md`, and `BILLING_API_BOUNDARY.md`; reserve the proof bundle. | Control plane, boundary, and ownership are explicit; proof pointer exists. |
| 02 | Product/pricing/referral/seat model | pending | `packages/billing-domain/src/billing-pricing-matrix.ts` | Read WP01 docs and the WP01 proof rows; keep pricing and referral math separate from billing transport. | Starter bundle, effective seat math, and over-limit behavior are proven. |
| 03 | Provider strategy and regional matrix | pending | `packages/billing-domain/src/billing-checkout-portal-boundary.ts` | Read WP08 and WP09 docs; confirm provider ordering and manual-required regions. | Provider order and regional matrix are closed. |
| 04 | Hosted checkout/customer portal/invoice model | pending | `packages/billing-domain/src/billing-checkout-portal-boundary.ts` | Read WP02 and WP05 docs; prove checkout, portal, and invoice flows with negative cases. | Redirect-success is not entitlement proof; invoice states are explicit. |
| 05 | Provider webhook lifecycle and idempotency | pending | `crates/billing-core/src/billing_subscription.rs` | Read WP03 docs and proof rows; capture replay, duplicate, and dead-letter evidence. | Signature, idempotency, and reconciliation are proven. |
| 06 | Referral qualification and anti-abuse lifecycle | pending | `packages/billing-domain/src/billing-entitlement.ts` | Read WP10 docs; keep household invites separate from referral credits. | Qualification, abuse rejection, and lost-credit behavior are proven. |
| 07 | App-owned billing/referral/entitlement ledgers | pending | `packages/billing-domain/src/billing-entitlement-runtime-proof.ts` | Read WP04 docs; ensure ledgers explain every seat without provider truth. | Ledger rows and projections explain access. |
| 08 | Signed EntitlementSnapshot and device-bound license gates | pending | `crates/entitlement-core/src/lib.rs` | Read WP04 snapshot docs; prove wrong-household and wrong-device rejection. | Snapshot signature and device binding are proven. |
| 09 | Parent website billing dashboard | pending | `packages/parent-domain/src/billing-entitlement.ts` | Read WP11 docs; keep the parent view redacted and billing-only. | Parent view shows billing state without child/private data. |
| 10 | Support/admin billing ops | pending | `packages/parent-domain/src/billing-support-admin-boundary.ts` | Read WP12 docs; keep audit, redaction, and support-role limits explicit. | Support/admin search and actions are audited and minimized. |
| 11 | Invoice/tax/refund/dispute/cancel/grace | pending | `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts` | Read WP05 docs; prove cancellation, grace, refunds, and disputes. | Invoice/grace behavior is explicit and replay-safe. |
| 12 | Security/privacy/observability/test-live boundary | pending | `packages/billing-domain/src/billing-security-privacy-observability.ts` | Read WP06 docs; keep secrets, logs, and test/live split clean. | No secret or child-data leakage; test/live split visible. |
| 13 | Rollout proof and route gate | pending | `docs/proof/payment-subscription-plan/07-validation-command-log.md` | Read WP07 docs; verify proof path, route sync, and validation logs. | Proof lives outside the plan folder and route docs are synced. |

## Working rules

- Move exactly one row to `in_progress` when execution starts.
- Do not start a new slice while a previous one still lacks proof or teardown evidence.
- If proof artifacts live inside the plan folder, move them out before claiming progress.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with the live queue and remaining manual-required gaps.
