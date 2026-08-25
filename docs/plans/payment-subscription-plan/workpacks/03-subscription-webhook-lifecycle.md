# Workpack 03: Subscription Webhook Lifecycle

## Current status

- Verdict: `blocked / source reviewed / runtime composition incomplete`
- Implementation phase: `blocked by WP02`; `graph:next -- --phase implementation` does not authorize WP03.
- Proof root: `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/` (absent in this checkout; no proof claim is made by this packet)
- Rust owner: `crates/billing-core/src/billing_subscription.rs`
- Reachable ingress owner: `infra/cloudflare/src/index.ts` and its Cloudflare route/auth boundary.
- No-claim boundary: this packet records reviewed source truth only. It does not claim a production Rust caller, provider execution, normalized lifecycle receipt truth, tests, proof, CI, PR, READY, or DONE.

The source review was performed on 2026-08-25 from the Payment WP02-integrated
base. The Rust lifecycle classifier/projector and the Cloudflare receipt,
queue, cursor, retry, dead-letter, and outbox primitives are real source, but
their production composition is incomplete.

## Goal

Define how provider events become app-owned billing truth, including signature validation, dedupe, retries, and reconciliation.

## Ownership boundary

```text
billing-core owns Rust provider lifecycle classification and event/idempotency helper behavior when selected.
payment-subscription-plan owns webhook-to-app-ledger semantics.
cloudflare-control-plane-plan owns shared Worker route/auth/runtime shell.
provider adapters must provide verified provider event formats; only the Stripe raw HMAC path is currently implemented.
entitlement delivery happens through WP04 and must not be bypassed by provider events.
```

## First-touch surface

- `crates/billing-core/src/billing_subscription.rs`
- `crates/billing-core/src/billing_subscription_webhook.rs`
- `crates/billing-core/src/billing_subscription_projection.rs`
- `crates/billing-core/src/billing_subscription_review.rs`
- `crates/billing-core/tests/unit/provider_webhook.rs`
- `crates/billing-core/tests/unit/subscription_lifecycle.rs`
- `infra/cloudflare/src/auth/provider-webhook.ts`
- `infra/cloudflare/src/auth/verifier.ts`
- `infra/cloudflare/src/routes.ts`
- `infra/cloudflare/src/index.ts`
- `infra/cloudflare/src/billing-binding-read-model.ts`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PLAN_EXECUTION_BLUEPRINT.md](../PLAN_EXECUTION_BLUEPRINT.md)
- [SUBSCRIPTION_WEBHOOK_LIFECYCLE.md](../SUBSCRIPTION_WEBHOOK_LIFECYCLE.md)
- [BILLING_API_BOUNDARY.md](../BILLING_API_BOUNDARY.md)
- [APP_OWNED_BILLING_LEDGER.md](../APP_OWNED_BILLING_LEDGER.md)

## Packet scope

This packet updates only the routed workpack and plan/graph truth. It does not
edit production source, tests, proof artifacts, CI, or PR state.

## Reviewed source truth

| Field | Current state | Evidence |
| --- | --- | --- |
| `provider` | Rust names Stripe, Razorpay, PayPal, Apple, and Google, and Cloudflare exposes five webhook routes; this is channel shape, not verified provider execution | `crates/billing-core/src/billing_subscription.rs`; `infra/cloudflare/src/routes.ts` |
| `mode` | Rust carries test/live mode as a caller-supplied enum; the Cloudflare receipt path does not persist or compose provider mode | `crates/billing-core/src/billing_subscription.rs`; `infra/cloudflare/src/billing-binding-read-model.ts` |
| `signature_state` | Stripe raw HMAC verification exists; Razorpay, PayPal, Apple, and Google fail closed as unavailable/manual-required; neither Cloudflare receipts nor the Rust ingress boundary receives normalized signature state | `infra/cloudflare/src/auth/provider-webhook.ts`; `infra/cloudflare/src/auth/verifier.ts`; `infra/cloudflare/src/index.ts` |
| `payload_parse_state` | Cloudflare rejects invalid JSON/non-object payloads, but the receipt schema does not persist parse state and no Rust lifecycle caller translates it | `infra/cloudflare/src/index.ts`; `infra/cloudflare/src/billing-binding-read-model.ts` |
| `idempotency_state` | D1/DO receipts, state-version guards, cursor CAS, queue leases, and outbox custody are real; Rust also classifies fresh/duplicate events, but no production caller composes the two | `infra/cloudflare/src/billing-binding-read-model.ts`; `crates/billing-core/src/billing_subscription.rs` |
| `replay_state` | Cloudflare rechecks receipt/provider/account references and cursor state; Rust replay semantics remain reachable only through caller-constructed events | `infra/cloudflare/src/index.ts`; `crates/billing-core/src/billing_subscription_webhook.rs` |
| `out_of_order_state` | Cloudflare has cursor/reconciliation follow-up primitives; there is no verified provider-event-to-Rust lifecycle bridge | `infra/cloudflare/src/index.ts`; `infra/cloudflare/src/billing-binding-read-model.ts`; `crates/billing-core/src/billing_subscription_review/reconciliation.rs` |
| `ledger_write_state` | The reachable queue path uses Cloudflare TypeScript mutation logic; no non-test caller invokes the Rust projector or establishes the app-owned ledger transition through it | `infra/cloudflare/src/index.ts`; `crates/billing-core/src/billing_subscription_projection.rs` |
| `retry_state` | Queue retry, lease, retry-exhaustion, and dead-letter paths exist in the Worker source; their provider verification and Rust lifecycle composition remain open | `infra/cloudflare/src/index.ts`; `infra/cloudflare/src/billing-binding-read-model.ts`; `crates/billing-core/src/billing_subscription_review/retry.rs` |
| `dead_letter_state` | Worker dead-letter/manual-required custody and Rust manual-review/dead-letter classifications exist, but no normalized receipt lifecycle reaches the Rust owner | `infra/cloudflare/src/index.ts`; `crates/billing-core/src/billing_subscription_review/dead_letter.rs` |
| `reconciliation_state` | Worker outbox/reconciliation custody and Rust reconciliation decisions exist as separate source surfaces; no production caller joins them | `infra/cloudflare/src/billing-binding-read-model.ts`; `crates/billing-core/src/billing_subscription_review/reconciliation.rs` |
| `test_live_boundary_state` | Rust rejects mixed mode when a caller supplies the enum; the reachable Cloudflare receipt does not carry provider mode, so this is not a production ingress guarantee | `crates/billing-core/src/billing_subscription.rs`; `infra/cloudflare/src/billing-binding-read-model.ts` |

The mapped Rust tests call the public classifier/projector with synthetic
caller-supplied provider, signature, parse, mode, account, idempotency, replay,
and ordering states. The mapped Cloudflare integration/fuzz tests use the
local-safe fixture harness, and some non-Stripe acceptance expectations are
stale against the current manual-required source. They are not live provider
proof, and no tests were run in this packet.

## Acceptance

- Every provider event is signature-verified before parsing.
- Every accepted event produces an app-owned ledger entry.
- Stable provider event IDs dedupe replays and duplicates.
- Out-of-order events converge on the same final ledger state.
- Retry or reconciliation work is queued when a webhook needs follow-up.

## Acceptance status

- [ ] Every production provider event has an owned verified signature boundary before parsing; only Stripe is implemented and other providers are manual-required.
- [ ] A real provider-event caller translates verified provider truth into the Rust lifecycle owner; no non-test caller exists.
- [ ] The durable receipt records signature state, payload parse state, provider mode, and the other required lifecycle fields; those fields are currently absent.
- [ ] The Cloudflare ingress, Account authority, Rust classifier/projector, and app-owned ledger are composed into one production path.
- [ ] Duplicate, replayed, out-of-order, retry, dead-letter, and reconciliation behavior has focused expected tests against the current source.
- [ ] The WP03 proof root and validation log exist; they are absent and remain open.
- [x] Provider events do not claim entitlement delivery completion; WP04 remains the entitlement-delivery owner.

The checked source facts above are bounded implementation evidence only; they
do not satisfy the unchecked production lifecycle gates.

## Required proof fields

The selected proof must name, at minimum:

```text
provider
mode
provider_event_id
signature_state
payload_parse_state
idempotency_state
replay_state
out_of_order_state
ledger_write_state
entitlement_update_requirement
retry_state
dead_letter_state
reconciliation_state
test_live_boundary_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Proof IDs

- `payment-webhook.stripe-signature-valid`
- `payment-webhook.razorpay-signature-valid`
- `payment-webhook.paypal-webhook-verified`
- `payment-webhook.duplicate-event-idempotent`
- `payment-webhook.replayed-event-rejected`
- `payment-webhook.reconciliation-repairs-drift`

## Validation

- This truth packet did not run tests, proof commands, CI, or production validation.
- Focused validation remains required after WP02/provider-owner integration: the Rust billing-core unit targets, the mapped Cloudflare webhook tests, architecture/source-shape/no-test-double/validation-bypass gates, and the proof-root validation log.
- Required proof families: `payment-webhook.stripe-signature-valid`, `payment-webhook.stripe-signature-invalid`, `payment-webhook.razorpay-signature-valid`, `payment-webhook.paypal-webhook-verified`, `payment-webhook.duplicate-event-idempotent`, `payment-webhook.replayed-event-rejected`, `payment-webhook.out-of-order-event-safe`, `payment-webhook.unknown-event-safe`, `payment-webhook.retry-no-double-grant`, `payment-webhook.dead-letter-manual-required`, `payment-webhook.reconciliation-repairs-drift`, `payment-webhook.test-live-separated`
- Proof bundle: expected but absent in this checkout: `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-provider-webhook-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-idempotency-replay-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-dead-letter-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-reconciliation-proof.md`, `output/payment-subscription-plan-proof/03-subscription-webhook-lifecycle/03-test-live-boundary-proof.md`

## Negative cases

- Reject unsigned or malformed provider payloads.
- Reject duplicate events as new entitlement.
- Reject replayed events that would double-grant access.
- Reject any webhook path that changes access without a ledger entry.
- Reject test/live mode mixing.

## Failure conditions

- Do not trust provider payloads without signature validation.
- Do not let a webhook change access without the ledger.
- Do not double-grant entitlement on duplicate events.
- Do not claim entitlement delivery from webhook acceptance alone; WP04 owns entitlement delivery proof.
