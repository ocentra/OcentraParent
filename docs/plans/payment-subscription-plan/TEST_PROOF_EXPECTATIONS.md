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

Run through `npm run agent:run --` when collecting proof if the wrapper is available.

## Required flow

- [ ] Select one workpack and its proof root from `PROOF_INDEX.md`.
- [ ] If the workpack is not WP00, confirm the Cloudflare prerequisite handoff or exact blocker first.
- [ ] Map the workpack to its exact required assertion IDs in `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- [ ] Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
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
cargo test -p ocentra-billing-core
cargo test -p ocentra-parent-billing-core
npm run test --workspace @ocentra-parent/portal -- billing
npm run lint:architecture -- --files infra/cloudflare packages/billing-domain crates/billing-core apps/portal docs/plans/payment-subscription-plan
```

## Command ownership notes

- `cloudflare-control-plane-plan` owns shared Cloudflare runtime module, auth, bindings, local dev/test, deploy, and payment handoff proof.
- `billing-domain` owns TypeScript billing/account/entitlement/support-admin/payment-facing contract and proof-consumer surfaces, but internal files are not public API proof by themselves.
- `billing-core` owns Rust provider lifecycle/event/idempotency helper behavior when selected.
- `schema-domain` owns canonical shared billing/payment/entitlement shapes when cross-boundary.
- Account, device-trust, data-custody, setup, portal, policy, enforcement, and child-data scopes run only when the selected workpack names the handoff.

## Payment E2E meaning

Do not use one proof family to claim the whole payment path. For this plan, E2E has separate meanings:

```text
Cloudflare handoff E2E: upstream module/auth/route/test handoff -> payment runtime remains blocked or accepted.
pricing/seat/referral E2E: plan/seat/referral inputs -> effective price/seat/credit math -> no provider transport claim.
checkout/portal E2E: server-created hosted session -> redirect/portal state -> no entitlement grant claim.
webhook lifecycle E2E: verified provider event -> idempotent ledger decision -> replay/out-of-order/dead-letter proof.
app-owned ledger E2E: provider/referral/billing facts -> app ledger rows -> entitlement projection.
signed entitlement snapshot E2E: app entitlement -> signed snapshot -> account/device-trust binding -> wrong-household/device rejection.
invoice/refund/dispute E2E: invoice/refund/dispute/cancel/grace event -> ledger transition -> audit/rollback proof.
provider portability E2E: provider-specific event/session -> normalized state -> manual-required provider gaps.
regional rollout E2E: region/currency/tax/provider availability -> launch blocker/allow state.
referral E2E: referral event -> qualification/review/lost-credit behavior -> entitlement recalculation.
parent dashboard E2E: parent-authorized view -> billing-only fields -> no child/private data proof.
support/admin E2E: support role -> bounded search/action -> audit/redaction proof.
rollout gate E2E: accepted roots + carried blockers -> route sync -> claims allowed/blocked.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Expected proof focus by workpack

| Workpack | Expected proof focus |
| --- | --- |
| WP00 | Cloudflare handoff accepted or blocker recorded |
| WP01 | pricing/seat/referral math, entitlement mapping, no hidden product unlock |
| WP02 | checkout/portal sessions, test/live separation, server-only provider secrets |
| WP03 | provider signature, event lifecycle, dedupe, retry, out-of-order convergence, dead-letter/manual-required |
| WP04 | app-owned ledger, signed entitlement snapshot, device/account handoff gates |
| WP05 | invoice/tax/refund/dispute/cancellation/grace behavior and audit refs |
| WP06 | privacy, provider metadata minimization, PCI boundary, review controls, observability |
| WP08 | provider adapter normalized state across Stripe/Razorpay/PayPal/store/manual |
| WP09 | region/currency/tax/provider availability matrix and launch blockers |
| WP10 | referral qualification, review controls, credit lifecycle, grace behavior |
| WP11 | parent-visible billing dashboard status, seats, invoices, self-service states |
| WP12 | support/admin search, refunds, disputes, adjustments, audit trail |
| WP07 | rollout proof pack, route sync, manual-required gap register |

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact provider secrets, webhook secrets, account tokens, card/payment instrument data, child private data, child telemetry, support-private notes, tax identifiers, raw provider payloads unless fixture-scoped, and live customer data
log workpack, provider, mode, region, account/household ref, ledger state, entitlement state, signature state, idempotency state, refund/dispute state, dashboard/support visibility state, rollback/teardown state, manual-required note, and no-claim boundary when safe
separate Cloudflare handoff, provider transport, app-owned ledger, entitlement snapshot, account authority, device trust, dashboard, support/admin, custody, and policy-consumer states
never treat redirects, provider events, assertion matrices, scaffold docs, or query output as runtime proof without selected proof root and no-claim boundaries
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, provider, mode, region, exit code, result, artifact pointer, diagnostics summary, blocker note, rollback/teardown pointer, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

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
billing-domain tests are not targeted parent-dashboard proof
assertion matrix completeness is not runtime proof
Cloudflare scaffold is not payment runtime proof
```

## Failure conditions

- Do not mark DONE or PR_READY until code/tests/validation/proof are complete for the selected runtime slice.
- Do not treat scaffold-only Cloudflare docs as payment runtime proof.
- Do not treat a complete assertion matrix as runtime proof.
- Do not store proof inventories inside this plan folder.
