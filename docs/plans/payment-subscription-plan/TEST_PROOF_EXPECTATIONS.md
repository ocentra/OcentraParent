<!-- agent-capsule -->

> Agent Capsule
> Plan: `payment-subscription-plan`
> Doc: `Payment Subscription Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: payment runtime completion without matching artifacts.

<!-- /agent-capsule -->

# Payment Subscription Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

## Required flow

- [ ] Select one workpack and its proof root from `PROOF_INDEX.md`.
- [ ] If the workpack is not WP00, confirm the Cloudflare prerequisite handoff or exact blocker first.
- [ ] Map the workpack to its exact required assertion IDs in `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- [ ] For docs/spec work, update route/proof/matrix/status docs without runtime claims.
- [ ] For runtime work, write/update code and tests for that workpack, including negative cases.
- [ ] Record exact validation commands or exact missing-runtime blockers.
- [ ] Record rollback or teardown note for the touched slice.
- [ ] Collect proof under `output/payment-subscription-plan-proof/<workpack-id>/`.
- [ ] Sync route, index, queue, and route-gate docs.

## Common command families

Use relevant commands only:

```bash
npm --prefix infra/cloudflare run test
npm --prefix infra/cloudflare run test:unit
npm --prefix infra/cloudflare run test:integration
npm --prefix infra/cloudflare run test:security
npm run build --workspace @ocentra-parent/billing-domain
npm run test --workspace @ocentra-parent/billing-domain
cargo test -p ocentra-parent-billing-core
npm run test --workspace @ocentra-parent/portal -- billing
npm run lint:architecture -- --files infra/cloudflare packages/billing-domain crates/billing-core apps/portal docs/plans/payment-subscription-plan
```

## Expected proof focus by workpack

| Workpack | Expected proof focus |
| --- | --- |
| WP00 | Cloudflare handoff accepted or blocker recorded |
| WP01 | pricing/seat/referral math, entitlement mapping, no hidden product unlock |
| WP02 | checkout/portal sessions, test/live separation, server-only provider secrets |
| WP03 | provider signature, event lifecycle, dedupe, retry, out-of-order convergence, dead-letter/manual-required |
| WP04 | app-owned ledger, signed entitlement snapshot, device/account handoff gates |
| WP05 | invoice/tax/refund/dispute/cancellation/grace behavior and audit refs |
| WP06 | privacy, provider metadata minimization, PCI boundary, abuse controls, observability |
| WP08 | provider adapter normalized state across Stripe/Razorpay/PayPal/store/manual |
| WP09 | region/currency/tax/provider availability matrix and launch blockers |
| WP10 | referral qualification, abuse controls, credit lifecycle, grace behavior |
| WP11 | parent-visible billing dashboard status, seats, invoices, self-service states |
| WP12 | support/admin search, refunds, disputes, adjustments, audit trail |
| WP07 | rollout proof pack, route sync, manual-required gap register |

## Required negative states

```text
checkout redirect is not paid access
provider event without server verification is blocked
same provider event cannot double grant
provider outage degrades safely
refund/dispute/cancel can remove or degrade entitlement correctly
provider metadata cannot include private child activity data
payment test mode and live mode cannot mix
entitlement cannot unlock without account and device-trust handoff
regional provider unavailable state is visible
support/admin action requires audit trail
```

## Failure conditions

- Do not mark DONE or PR_READY until code/tests/validation/proof are complete for the selected runtime slice.
- Do not treat scaffold-only Cloudflare docs as payment runtime proof.
- Do not treat a complete assertion matrix as runtime proof.
- Do not store proof inventories inside this plan folder.
