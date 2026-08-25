# Workpack 05: Invoice, Tax, Refund, and Dispute

## Goal

Define invoice, tax, refund, dispute, cancellation, and grace behavior as one coordinated contract.

## Planned first-touch surface

- `packages/billing-domain/src/billing-invoice-tax-refund-dispute.ts`
- `packages/billing-domain/tests/unit/billing-invoice-tax-refund-dispute.test.ts`

Both planned `packages/billing-domain` paths are absent in the reviewed
checkout. They remain planned implementation and expected-test gaps; this
workpack does not recreate that package from schema or fixture code.

## Reviewed production truth - 2026-08-25

This source-and-routing checkpoint does not add completion evidence, tests,
proof, CI, PR, READY, or DONE state.

The actual production boundary is Cloudflare-owned.
`infra/cloudflare/src/billing-binding-read-model.ts` contains durable
D1/Durable Object state, mutation idempotency, mutation outbox delivery,
invoice/admin-invoice rows, admin-dispute rows, and a refund ledger.
`applyBillingStateMutation` validates invoice identity, currency, cumulative
refund amount, refund state, and replay/terminal guards before committing the
app-owned state transition. Its manual-invoice, admin-refund, cancellation,
and provider-webhook branches remain explicit app-state projections; they do
not call a provider API or establish a provider-owned execution owner.

The reachable Worker callers in `infra/cloudflare/src/index.ts` are narrower:
parent invoices and support/admin invoice and dispute reads call the durable
read model; the admin refund handler returns `manual-required`, and the parent
cancel and manual-invoice handlers are also manual-required. The route manifest
keeps the invoice request/response and mutation request codecs unbound where
the generated boundary is missing, with blockers including
`billing-refund-owner-adapter-missing`,
`payment-provider-execution-owner-missing`, and
`manual-invoice-owner-adapter-missing`. The provider-webhook queue consumer can
apply a verified, authority-matched receipt to the Cloudflare mutation path,
but WP03's missing normalized provider receipt/lifecycle owner composition
still prevents treating that path as a complete invoice, tax, refund, or
dispute lifecycle.

`crates/billing-core/src/billing_subscription.rs` exposes lifecycle helpers
through a public module, but its projection module is private to the crate and
no non-test production caller of the lifecycle API was found. The Rust schema
and generated schema-domain files are export/contract surfaces; their contract
tests compare generated files and do not prove provider execution or a live
ledger caller. The mapped Cloudflare tests use `createTestHarness` with
`ENVIRONMENT: 'test'` and `AUTH_ADAPTER_MODE: 'local-safe-fixture'`; they cover
local accepted/rejected route contracts and durable-state shapes, not live
provider execution or completion of the full lifecycle matrix.

The required assertion matrix remains open for all 16 rows, including invoice
and receipt visibility, tax mode, full/partial/failed refund, dispute
resolution and chargeback, grace, both cancellation modes, resume, audited
support/admin action, and no-data-delete-on-refund. The WP05 proof directory is
absent. WP05 therefore remains `blocked / source reviewed`: the real
Cloudflare state boundary is recorded, while provider execution/owner
composition, normalized lifecycle authority, the planned billing-domain
source/test paths, focused expected tests, and proof remain open.

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [INVOICE_TAX_REFUND_DISPUTE_MODEL.md](../INVOICE_TAX_REFUND_DISPUTE_MODEL.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [PAYMENT_PROVIDER_STRATEGY.md](../PAYMENT_PROVIDER_STRATEGY.md)

## Output files

- [INVOICE_TAX_REFUND_DISPUTE_MODEL.md](../INVOICE_TAX_REFUND_DISPUTE_MODEL.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)
- `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/`

## Target acceptance (not met by this truth packet)

- Invoice lifecycle records plan, period, currency, tax, provider reference, and invoice number.
- Refunds preserve history and record the entitlement impact.
- Disputes freeze or limit entitlement per policy and retain evidence pointers.
- Cancellation and grace are visible in the entitlement ledger and snapshot model.
- Manual invoice state remains auditable and support-owned.

## Initial proof IDs (incomplete; not produced)

- `payment-lifecycle.invoice-visible`
- `payment-lifecycle.refund-state`
- `payment-lifecycle.dispute-opened`
- `payment-lifecycle.failed-renewal-grace`

## Validation (deferred)

- Docs validation: `npm run format:check`; `npm run lint:schema-boundaries`
- This truth packet runs no tests, proof, CI, or precommit. The commands above
  remain required for the implementation/test/proof wave.
- Required proof families: `payment-lifecycle.invoice-visible`, `payment-lifecycle.receipt-visible`, `payment-lifecycle.tax-mode-decision`, `payment-lifecycle.refund-state`, `payment-lifecycle.partial-refund-state`, `payment-lifecycle.refund-failed-state`, `payment-lifecycle.dispute-opened`, `payment-lifecycle.dispute-won`, `payment-lifecycle.dispute-lost`, `payment-lifecycle.chargeback-state`, `payment-lifecycle.failed-renewal-grace`, `payment-lifecycle.cancel-immediate`, `payment-lifecycle.cancel-period-end`, `payment-lifecycle.resume-after-past-due`, `payment-lifecycle.support-admin-audited`, `payment-lifecycle.no-data-delete-on-refund`
- Proof bundle: `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-tax-refund-dispute-matrix.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-invoice-dashboard-proof.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-refund-dispute-entitlement-proof.md`, `output/payment-subscription-plan-proof/05-invoice-tax-refund-dispute/05-support-admin-audit-proof.md`

## Negative cases

- Reject any refund or dispute that changes access without a ledger entry.
- Reject any invoice or tax record that contains child telemetry.
- Reject provider invoice events as final until the app ledger records them.
- Reject silent cancellation or grace changes that are not visible in the ledger.

## Failure conditions

- Do not allow a refund or dispute to silently change access without a ledger entry.
- Do not let tax or invoice data contain child telemetry.
- Do not treat a provider invoice event as final until the app ledger records it.
