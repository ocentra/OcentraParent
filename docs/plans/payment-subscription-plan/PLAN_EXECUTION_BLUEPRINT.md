# Execution Blueprint

Status: reset.

## Execution objective

Turn the monetization route from docs into code, tests, validation, and proof in slice order.

## Execution slices

1. Cloudflare billing control plane.
2. Product/pricing/referral/seat model.
3. Provider strategy and regional matrix.
4. Hosted checkout/customer portal/invoice model.
5. Provider webhook lifecycle and idempotency.
6. Referral qualification and anti-abuse lifecycle.
7. App-owned billing/referral/entitlement ledgers.
8. Signed EntitlementSnapshot and device-bound license gates.
9. Parent website billing dashboard.
10. Support/admin billing ops.
11. Invoice/tax/refund/dispute/cancel/grace.
12. Security/privacy/observability/test-live boundary.
13. Rollout proof and route gate.

## Required order

- Select one workpack, then collect code, tests, validation, proof, and route sync for that workpack only.
- Do not mix proof from sibling workpacks into the current proof folder.
- Do not mark a workpack PR-ready until the selected slice has at least one negative test and one rollback or teardown proof.
