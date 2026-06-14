# Execution Blueprint

Status: reset.

## Execution objective

Turn the monetization route from docs into code, tests, validation, and proof in slice order.

## Slice gates

| Slice | Primary docs/workpacks | First-touch source surface | Entry criteria | Exit criteria | Proof pointer | Rollback / teardown |
|---|---|---|---|---|---|---|
| 01. Cloudflare billing control plane | `AGENTS.md`, `PLAN_STATE.md`, `CLOUDFLARE_BILLING_CONTROL_PLANE.md`, `BILLING_API_BOUNDARY.md`, `DECISIONS.md` | `packages/billing-domain/src/billing-account-runtime-boundary.ts`; `packages/billing-domain/src/billing-checkout-portal-boundary.ts` | Route docs read; control-plane boundary selected; no unresolved sibling-plan handoff | Control plane shape is explicit; provider secrets and Worker/DO boundary are named; adjacent plan handoffs are linked | `docs/proof/payment-subscription-plan/PLAN_PROOF_MANIFEST.md` | Restore the prior route state in `PLAN_STATE.md` if the control-plane boundary changes |
| 02. Product/pricing/referral/seat model | WP01 | `packages/billing-domain/src/billing-pricing-matrix.ts`; `packages/billing-domain/src/billing-entitlement.ts` | Pricing, starter bundle, referral, and entitlement docs loaded | Effective seat math, lost-referral behavior, and over-limit grace are defined with failure cases | `docs/proof/payment-subscription-plan/01-free-starter-bundle-proof.md` and related WP01 bundles | Teardown proof must show no child data or entitlement loss |
| 03. Provider strategy and regional matrix | WP08, WP09 | `packages/billing-domain/src/billing-checkout-portal-boundary.ts`; `packages/billing-domain/src/billing-account-runtime-boundary.ts` | Provider and regional docs loaded; manual-required regions named | Provider authority, fallback order, and region matrix are explicit | `docs/proof/payment-subscription-plan/08-provider-adapter-contract-proof.md`, `docs/proof/payment-subscription-plan/09-regional-payment-matrix.md` | Roll back by reverting unsupported-region claims to manual-required |
| 04. Hosted checkout/customer portal/invoice model | WP02, WP05 | `packages/billing-domain/src/billing-checkout-portal-boundary.ts`; `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts` | Billing API boundary and portal docs loaded | Checkout, portal, and invoice semantics are server-side and redaction-safe | `docs/proof/payment-subscription-plan/02-hosted-checkout-proof.md`, `docs/proof/payment-subscription-plan/05-invoice-dashboard-proof.md` | Teardown proof must show redirect success does not imply payment completion |
| 05. Provider webhook lifecycle and idempotency | WP03 | `crates/billing-core/src/billing_subscription.rs`; `crates/billing-core/tests/unit/provider_webhook.rs` | Webhook lifecycle doc loaded; provider signatures and dedupe rules named | Signature verification, duplicate handling, replay rejection, and reconciliation are explicit | `docs/proof/payment-subscription-plan/03-provider-webhook-proof.md`, `docs/proof/payment-subscription-plan/03-idempotency-replay-proof.md` | Roll back by showing dead-letter or retry without double-grant |
| 06. Referral qualification and anti-abuse lifecycle | WP10 | `packages/billing-domain/src/billing-entitlement.ts`; `packages/billing-domain/src/billing-entitlement-runtime-proof.ts` | Referral ledger and anti-abuse rules loaded | Qualification, abuse, revocation, and grace states are explicit | `docs/proof/payment-subscription-plan/10-referral-state-machine-proof.md`, `docs/proof/payment-subscription-plan/10-referral-abuse-negative-proof.md` | Teardown proof must preserve history after lost credit or revocation |
| 07. App-owned billing/referral/entitlement ledgers | WP04 | `packages/billing-domain/src/billing-entitlement.ts`; `packages/billing-domain/src/billing-entitlement-runtime-proof.ts` | Ledger models loaded | Billing, referral, and entitlement rows explain every active seat | `docs/proof/payment-subscription-plan/04-entitlement-ledger-proof.md` | Roll back by restoring ledger-source authority from provider state |
| 08. Signed EntitlementSnapshot and device-bound license gates | WP04 | `crates/entitlement-core/src/lib.rs`; `crates/entitlement-core/tests/unit/capability_gate.rs` | Snapshot model loaded; device-trust handoff named | Snapshot fields, signature checks, and device-binding rejection are explicit | `docs/proof/payment-subscription-plan/04-signed-snapshot-proof.md`, `docs/proof/payment-subscription-plan/04-local-device-trust-required-proof.md` | Teardown proof must show wrong-household and wrong-device rejection |
| 09. Parent website billing dashboard | WP11 | `packages/parent-domain/src/billing-entitlement.ts`; `packages/parent-domain/src/billing-entitlement-proof.ts` | Parent dashboard docs loaded; redaction boundary named | Parent billing view, change-plan, cancel, portal, and license states are explicit | `docs/proof/payment-subscription-plan/11-parent-website-dashboard-proof.md` | Roll back by removing any child/private data from the parent surface |
| 10. Support/admin billing ops | WP12 | `packages/parent-domain/src/billing-support-admin-boundary.ts`; `packages/parent-domain/src/billing-support-admin-status-proof.ts` | Support/admin docs loaded; audit boundary named | Admin search, refunds, disputes, manual invoices, and reconciliation are explicit | `docs/proof/payment-subscription-plan/12-support-admin-ops-proof.md` | Teardown proof must show support cannot see child/private data |
| 11. Invoice/tax/refund/dispute/cancel/grace | WP05 | `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts`; `packages/billing-domain/tests/unit/billing-invoice-tax-refund-dispute.test.ts` | Invoice/grace docs loaded; manual-required states named | Final tax, refunds, disputes, cancellation, and grace behavior are explicit | `docs/proof/payment-subscription-plan/05-refund-dispute-entitlement-proof.md`, `docs/proof/payment-subscription-plan/05-support-admin-audit-proof.md` | Roll back by proving refund/dispute/cancel never changed access without a ledger entry |
| 12. Security/privacy/observability/test-live boundary | WP06 | `packages/billing-domain/src/billing-security-privacy-observability.ts`; `packages/billing-domain/tests/unit/billing-security-privacy-observability.test.ts` | Security/privacy docs loaded; test/live split named | Secret handling, redaction, abuse gating, and observability are explicit | `docs/proof/payment-subscription-plan/06-secret-scan-proof.md`, `docs/proof/payment-subscription-plan/06-support-view-minimized-proof.md` | Teardown proof must show no secret or child-data leak in logs or metadata |
| 13. Rollout proof and route gate | WP07 | `docs/proof/payment-subscription-plan/07-validation-command-log.md`; `scripts/test/real-evidence-proof-checkpoint.mjs` | Proof manifest, blueprint, and next actions loaded | Proof location, validation command family, and route sync are explicit | `docs/proof/payment-subscription-plan/07-route-sync-proof.md`, `docs/proof/payment-subscription-plan/07-validation-command-log.md` | Roll back by restoring route/index state and proof manifest alignment |

## Required order

1. Select exactly one slice/workpack.
2. Read the slice docs and the matching proof rows only.
3. Write or update the smallest scope for that slice.
4. Run the validation command family for that slice.
5. Collect negative-case proof and teardown proof.
6. Update the proof bundle pointer, blueprint, and next-actions queue.
7. Do not mark PR-ready until the proof bundle exists and the negative case is captured.

## Stop rules

- Do not mix proof from sibling workpacks into the current proof bundle.
- Do not advance to the next slice until the current slice has a proof pointer and teardown evidence.
- Do not mark a slice complete if the proof manifest or next-actions queue still names stale paths.
