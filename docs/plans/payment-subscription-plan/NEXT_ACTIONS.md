# Next Actions

## Current slice

- Current slice: `00. Cloudflare control-plane prerequisite handoff`
- Current owner: `payment-subscription-plan`
- Current status: `pending`

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
|---|---|---|---|---|---|
| 00 | Cloudflare control-plane prerequisite handoff | pending | `docs/plans/cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md` | Read WP00 plus the Cloudflare plan's parity map, auth boundary, route manifest, testing strategy, and handoff gate. | Cloudflare plan exists, payment handoff proof path is explicit, and payment runtime no-claim boundaries are recorded. |
| 01 | Cloudflare billing control-plane overlay | pending | `packages/billing-domain/src/billing-account-runtime-boundary.ts` | Read `CLOUDFLARE_BILLING_CONTROL_PLANE.md`, `BILLING_API_BOUNDARY.md`, and `SOURCE_SURFACE_STATUS_MATRIX.md`; keep shared module ownership out of payment. | Billing overlay, source reality, and proof pointer are explicit. |
| 02 | Product/pricing/referral/seat model | pending | `packages/billing-domain/src/billing-pricing-matrix.ts` | Read WP01 docs and proof rows; keep pricing and referral math separate from billing transport. | Starter bundle, effective seat math, and over-limit behavior are explicit. |
| 03 | Provider strategy and regional matrix | pending | `packages/billing-domain/src/billing-checkout-portal-boundary.ts` | Read WP08 and WP09 docs; confirm provider ordering and manual-required regions. | Provider order and regional matrix are explicit. |
| 04 | Hosted checkout/customer portal/invoice model | pending | `packages/billing-domain/src/billing-checkout-portal-boundary.ts` | Read WP02 and WP05 docs; prove checkout, portal, and invoice flows with negative cases. | Redirect-success is not entitlement proof; invoice states are explicit. |
| 05 | Provider webhook lifecycle and idempotency | pending | `crates/billing-core/src/billing_subscription.rs` | Read WP03 docs and proof rows; capture replay, duplicate, and dead-letter evidence. | Signature, idempotency, and reconciliation are explicit. |
| 06 | Referral qualification and anti-abuse lifecycle | pending | `packages/billing-domain/src/billing-entitlement.ts` | Read WP10 docs; keep household invites separate from referral credits. | Qualification, abuse rejection, and lost-credit behavior are explicit. |
| 07 | App-owned billing/referral/entitlement ledgers | pending | `packages/billing-domain/src/billing-entitlement-runtime-proof.ts` | Read WP04 docs; ensure ledgers explain every seat without provider truth. | Ledger rows and projections explain access. |
| 08 | Signed EntitlementSnapshot and device-bound license gates | pending | `crates/entitlement-core/src/lib.rs` | Read WP04 snapshot docs; prove wrong-household and wrong-device rejection. | Snapshot signature and device binding are explicit. |
| 09 | Parent website billing dashboard | pending | `packages/parent-domain/src/billing-entitlement.ts` | Read WP11 docs; keep the parent view redacted and billing-only. | Parent view shows billing state without child/private data; missing targeted proof test stays open until added. |
| 10 | Support/admin billing ops | pending | `packages/parent-domain/src/billing-support-admin-boundary.ts` | Read WP12 docs; keep audit, redaction, and support-role limits explicit. | Support/admin search and actions are audited and minimized. |
| 11 | Invoice/tax/refund/dispute/cancel/grace | pending | `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts` | Read WP05 docs; prove cancellation, grace, refunds, and disputes. | Invoice/grace behavior is explicit and replay-safe. |
| 12 | Security/privacy/observability/test-live boundary | pending | `packages/billing-domain/src/billing-security-privacy-observability.ts` | Read WP06 docs; keep secrets, logs, and test/live split clean. | No secret or child-data leakage; test/live split visible. |
| 13 | Rollout proof and route gate | pending | `docs/proof/payment-subscription-plan/wp07-rollout-proof-and-route-gate/07-validation-command-log.md` | Read WP07 docs; verify proof path, route sync, and validation logs. | Proof lives outside the plan folder and route docs are synced. |

## Working rules

- Move exactly one row to `in_progress` when execution starts.
- Do not start a payment runtime slice while row 00 still lacks handoff proof.
- If proof artifacts live inside the plan folder, move them out before claiming progress.
- Keep `PLAN_EXECUTION_SCORECARD.md` and `SOURCE_SURFACE_STATUS_MATRIX.md` aligned with the live queue and remaining manual-required gaps.
