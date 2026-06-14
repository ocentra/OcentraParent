# Proof Manifest - payment-subscription-plan

Purpose: map each payment-subscription workpack to its proof bundle, validation command family, and required negative-case evidence.

## Scope

- Plan route: [PLAN_STATE.md](../../plans/payment-subscription-plan/PLAN_STATE.md)
- Workpacks: [WORKPACK_INDEX.md](../../plans/payment-subscription-plan/WORKPACK_INDEX.md)
- Execution order: [PLAN_EXECUTION_BLUEPRINT.md](../../plans/payment-subscription-plan/PLAN_EXECUTION_BLUEPRINT.md)
- Inventory: [PROOF_AND_TEST_INVENTORY.md](../../plans/payment-subscription-plan/PROOF_AND_TEST_INVENTORY.md)

## Required proof bundle map

| Workpack | Proof bundle paths | Validation command family | Negative evidence required |
|---|---|---|---|
| WP01 Product Pricing and Entitlement | `docs/proof/payment-subscription-plan/01-free-starter-bundle-proof.md`, `docs/proof/payment-subscription-plan/01-effective-child-device-limit-proof.md`, `docs/proof/payment-subscription-plan/01-safety-critical-grace-proof.md`, `docs/proof/payment-subscription-plan/01-rejected-game-economy-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Negative seat counts, overflow entitlement, and rejected game-economy model |
| WP02 Checkout and Billing Portal | `docs/proof/payment-subscription-plan/02-cloudflare-billing-api-boundary-proof.md`, `docs/proof/payment-subscription-plan/02-hosted-checkout-proof.md`, `docs/proof/payment-subscription-plan/02-billing-portal-proof.md`, `docs/proof/payment-subscription-plan/02-no-client-secret-proof.md`, `docs/proof/payment-subscription-plan/02-redirect-origin-negative-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Auth-required failure, origin/CSRF failure, bot-abuse gate, redirect-not-entitlement proof |
| WP03 Subscription Webhook Lifecycle | `docs/proof/payment-subscription-plan/03-provider-webhook-proof.md`, `docs/proof/payment-subscription-plan/03-idempotency-replay-proof.md`, `docs/proof/payment-subscription-plan/03-dead-letter-proof.md`, `docs/proof/payment-subscription-plan/03-reconciliation-proof.md`, `docs/proof/payment-subscription-plan/03-test-live-boundary-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Invalid signature, replay/duplicate, out-of-order state, dead-letter, test/live separation |
| WP04 Entitlement Delivery Gates | `docs/proof/payment-subscription-plan/04-entitlement-ledger-proof.md`, `docs/proof/payment-subscription-plan/04-signed-snapshot-proof.md`, `docs/proof/payment-subscription-plan/04-local-device-trust-required-proof.md`, `docs/proof/payment-subscription-plan/04-referral-loss-recalculation-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Wrong-household rejection, wrong-device rejection, stale snapshot rejection |
| WP05 Invoice, Tax, Refund, and Dispute | `docs/proof/payment-subscription-plan/05-invoice-tax-refund-dispute-matrix.md`, `docs/proof/payment-subscription-plan/05-invoice-dashboard-proof.md`, `docs/proof/payment-subscription-plan/05-refund-dispute-entitlement-proof.md`, `docs/proof/payment-subscription-plan/05-support-admin-audit-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Partial refund, dispute open/close, cancellation/grace, no-data-delete-on-refund |
| WP06 Security, Privacy, and Observability | `docs/proof/payment-subscription-plan/06-metadata-privacy-proof.md`, `docs/proof/payment-subscription-plan/06-secret-scan-proof.md`, `docs/proof/payment-subscription-plan/06-referral-abuse-proof.md`, `docs/proof/payment-subscription-plan/06-support-view-minimized-proof.md`, `docs/proof/payment-subscription-plan/06-pci-hosted-boundary-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Metadata allow/deny, webhook smuggling/replay, open-redirect negative, PCI boundary |
| WP07 Rollout, Proof, and Route Gate | `docs/proof/payment-subscription-plan/07-route-sync-proof.md`, `docs/proof/payment-subscription-plan/07-proof-path-proof.md`, `docs/proof/payment-subscription-plan/07-validation-command-log.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Stale route index, proof-in-plan-folder failure, rollback/teardown proof |
| WP08 Provider Adapter Portability | `docs/proof/payment-subscription-plan/08-provider-adapter-contract-proof.md`, `docs/proof/payment-subscription-plan/08-normalized-event-proof.md`, `docs/proof/payment-subscription-plan/08-provider-lock-escape-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Adapter lock escape, direct product-provider read, missing config fallback |
| WP09 Regional Payment Rollout | `docs/proof/payment-subscription-plan/09-regional-payment-matrix.md`, `docs/proof/payment-subscription-plan/09-india-razorpay-proof.md`, `docs/proof/payment-subscription-plan/09-pakistan-manual-required-proof.md`, `docs/proof/payment-subscription-plan/09-china-wallet-proof.md`, `docs/proof/payment-subscription-plan/09-uae-provider-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Unsupported region, missing matrix entry, untested tax/currency behavior |
| WP10 Referral Growth and Entitlement | `docs/proof/payment-subscription-plan/10-referral-state-machine-proof.md`, `docs/proof/payment-subscription-plan/10-referral-qualification-proof.md`, `docs/proof/payment-subscription-plan/10-referral-abuse-negative-proof.md`, `docs/proof/payment-subscription-plan/10-referral-loss-entitlement-proof.md`, `docs/proof/payment-subscription-plan/10-over-limit-grace-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Self-referral, same-household, same-device-farm, manual-review, no-data-delete-on-lost-referral |
| WP11 Parent Website Billing Dashboard | `docs/proof/payment-subscription-plan/11-parent-website-dashboard-proof.md`, `docs/proof/payment-subscription-plan/11-dashboard-wrong-household-negative-proof.md`, `docs/proof/payment-subscription-plan/11-dashboard-no-child-private-data-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Wrong-household denial, no-child-private-data proof, portal handoff omission |
| WP12 Support/Admin Billing Ops | `docs/proof/payment-subscription-plan/12-support-admin-ops-proof.md`, `docs/proof/payment-subscription-plan/12-admin-role-negative-proof.md`, `docs/proof/payment-subscription-plan/12-support-data-minimization-proof.md`, `docs/proof/payment-subscription-plan/12-reconciliation-admin-proof.md` | `npm run format:check`; `npm run lint:schema-boundaries` | Admin role required, support-role limited, audit-event required, no-child-private-data |

## Run log template

- date: YYYY-MM-DD
- workpack: WP##
- command: ...
- result: pass | fail
- negative case covered: ...
- proof bundle path: ...
- teardown or rollback evidence: ...
- follow-up command(s): ...

## Slice close gate

1. Attach the proof bundle for the selected workpack.
2. Include at least one negative-case proof.
3. Include teardown or rollback evidence.
4. Record the validation command family and log output.
5. Update the blueprint and next-actions docs only after the proof bundle exists.
