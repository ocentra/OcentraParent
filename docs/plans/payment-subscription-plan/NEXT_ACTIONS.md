# Payment Subscription Next Actions

## Current slice

- Current slice: `accepted production source -> Account migration handoff -> expected-test wave`
- Current owner: `payment-subscription-plan` with `account-identity-family-plan` dependency
- Current status: `source integrated / expected tests open / issuer manual-required`

The 2026-08-17 source checkpoint removes caller-minted entitlement authority,
adds durable lease/retry/collision handling for billing mutations, and adds the
legacy-to-canonical provider-mapping migration. Do not start proof or PR work
from that checkpoint. First integrate the Account WP02 current-authority
migration, then write the complete Payment expected-test delta (including
removed signed-API rewrites, migration ambiguity/collision cases, lease expiry,
retry exhaustion, crash recovery, and no-double-provider-effect assertions).

## Ordered queue

Queue statuses below describe runtime execution order. They do not reduce the engineering-spec score of this plan.

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
|---|---|---|---|---|---|
| 00 | Cloudflare control-plane prerequisite handoff | blocked / proof-present | `docs/plans/cloudflare-control-plane-plan/PARENT_CLOUDFLARE_MODULE_SPEC.md` | Carry the exact upstream Cloudflare blocker set forward; keep broader runtime payment claims blocked while allowing WP01 pricing proof to remain a no-runtime model slice. | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md` and `output/payment-subscription-plan-proof/00-cloudflare-control-plane-handoff/` both exist, and payment runtime remains explicitly blocked until the upstream Cloudflare blockers clear. |
| 01 | Cloudflare billing control-plane overlay | blocked / proof-present | `packages/schema-domain/src/billing-account-runtime-boundary.ts`; `packages/schema-domain/src/endpoint-billing-account.ts` | Local boundary proof is now present: keep shared module ownership out of payment, keep provider/backend/runtime non-claims explicit, and carry only the current broader workspace validation blockers. | `output/payment-subscription-plan-proof/02-checkout-billing-portal/02-cloudflare-billing-api-boundary-proof.md` exists and keeps Cloudflare/backend runtime out of payment ownership. |
| 02 | Product/pricing/referral/seat model | source present / runtime owner and expected tests open | `crates/entitlement-core/src/entitlement_snapshot.rs`; `crates/entitlement-core/src/entitlement_snapshot_derivation.rs` | Preserve the checked Rust base/referral/paid-seat derivation and unsigned-only projection. First supply the durable provider billing/referral ledger plus Account, Device-Trust, and issuer composition; then add the absent focused derivation/negative tests. Do not restore `packages/billing-domain` or treat generated edge contracts as pricing authority. | A real non-test owner consumes ledger state and issues only through trusted Account/Device-Trust authority; expected tests pass. Proof remains deferred until then. |
| 03 | Provider strategy and regional matrix | pending | `packages/schema-domain/src/billing-checkout-portal-boundary.ts` | Read WP08 and WP09 docs; confirm provider ordering and manual-required regions. | Provider order and regional matrix are explicit. |
| 04 | Hosted checkout/customer portal/invoice model | blocked / proof-present | `packages/schema-domain/src/billing-checkout-portal-boundary.ts`; `packages/schema-domain/src/billing-checkout-portal-boundary-values.ts`; `packages/billing-domain/tests/unit/billing-checkout-portal-boundary.test.ts`; `scripts/test/payment-checkout-boundary-proof.mjs` | Completed local boundary proof: keep hosted checkout and portal return states explicit, keep redirect/auth/origin/csrf/secret negatives explicit, keep redirect success non-entitlement, and carry the exact repo-wide validation blockers until global gates are repaired. | `output/payment-subscription-plan-proof/02-checkout-billing-portal/` exists with hosted checkout, billing portal, no-client-secret, negative-case, rollback/teardown, and validation-command proof. |
| 05 | Provider webhook lifecycle and idempotency | source integrated / expected tests open | `crates/billing-core/src/billing_subscription.rs`; `infra/cloudflare/src/index.ts` | Durable Worker mutation ownership, lease/retry recovery, stale cursor rejection, and outbox replay are now source-integrated. Write the complete crash/retry/collision/no-double-effect test delta after Account authority lands; keep missing provider adapters manual-required. | Focused expected tests exist and later pass against the accepted source without inventing provider authority. |
| 06 | Referral qualification and anti-abuse lifecycle | pending | `packages/billing-domain/src/billing-entitlement.ts` | Read WP10 docs; keep household invites separate from referral credits. | Qualification, review, and lost-credit behavior are explicit. |
| 07 | App-owned billing/referral/entitlement ledgers | source integrated / expected tests open | `crates/entitlement-core/src/entitlement_access.rs`; `infra/cloudflare/src/billing-binding-read-model.ts` | Provider state is input-only and accepted mutations are durably serialized. Complete the expected-test delta and Account-authority composition before claiming a live ledger path. | Focused ledger/replay tests later pass; proof remains deferred. |
| 08 | Entitlement projection and device-bound license gates | source integrated / issuer blocked / expected tests open | `crates/entitlement-core/src/entitlement_snapshot.rs`; `crates/entitlement-core/tests/contract/signed_snapshot_delivery.rs` | Caller-supplied signing material was removed. Keep the projection unsigned/manual-required until a real issuer/verifier exists, and rewrite stale signed-API tests in the expected-test wave. | No caller can mint signed authority; issuer-owned tests exist only after an issuer boundary is implemented. |
| 09 | Parent website billing dashboard | pending | `packages/parent-domain/src/billing-entitlement.ts` | Read WP11 docs; keep the parent view redacted and billing-only. | Parent view shows billing state without child/private data; the targeted proof test exists, but parent-domain build/import failure still blocks execution. |
| 10 | Support/admin billing ops | pending | `packages/parent-domain/src/billing-support-admin-boundary.ts` | Read WP12 docs; keep audit, redaction, and support-role limits explicit. | Support/admin search and actions are audited and minimized. |
| 11 | Invoice/tax/refund/dispute/cancel/grace | pending | `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts` | Read WP05 docs; prove cancellation, grace, refunds, and disputes. | Invoice/grace behavior is explicit and replay-safe. |
| 12 | Security/privacy/observability/test-live boundary | pending | `packages/billing-domain/src/billing-security-privacy-observability.ts` | Read WP06 docs; keep secrets, logs, and test/live split clean. | No secret or child-data leakage; test/live split visible. |
| 13 | Rollout proof and route gate | pending | `output/payment-subscription-plan-proof/07-rollout-proof-and-route-gate/07-validation-command-log.md` | Read WP07 docs; verify proof path, route sync, and validation logs. | Proof lives outside the plan folder and route docs are synced. |

## Working rules

- Move exactly one row to `in_progress` when execution starts.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.
- Do not start a payment runtime slice while row 00 still lacks handoff proof.
- If proof artifacts live inside the plan folder, move them out before claiming progress.
- Do not shrink, merge away, or reinterpret test scope outside `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md`, `SOURCE_SURFACE_STATUS_MATRIX.md`, `WORKPACK_INDEX.md`, and proof routes aligned with the live queue and remaining manual-required gaps.
- Treat `packages/schema-domain/src/**`, `packages/billing-domain/src/**`, and `packages/parent-domain/src/**` first-touch files as implementation or edge-contract targets, not public runtime proof, unless public exports and focused proof artifacts are selected.
