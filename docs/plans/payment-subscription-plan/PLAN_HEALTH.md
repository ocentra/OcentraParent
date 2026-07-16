# Payment Subscription Plan Health

Health: execution-grade route exists; implementation, adapter parity, and proof remain open.

Known risks: provider-region mismatch, webhook replay, referral review gaps, entitlement drift, device-trust dependency, child-data metadata leak, dashboard authorization gaps, refund/dispute edge cases, and test/live mode confusion.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist/proof row if one exists.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and `SOURCE_SURFACE_STATUS_MATRIX.md` if current state changes.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not claim READY from checkout redirect success.
- Do not claim READY from Cloudflare scaffold or route presence.
- Do not claim READY from provider events without server verification, idempotency, replay, and ledger proof.
- Do not claim READY from provider state without app-owned ledger projection.
- Do not claim READY from entitlement snapshots without account and device-trust handoff proof.
- Do not claim READY from billing-domain tests as parent dashboard proof.
- Do not claim READY from support/admin surfaces without role, redaction, and audit proof.
- Do not claim READY from assertion matrix completeness, provider docs, or regional availability docs.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Known healthy boundaries

This plan intentionally separates:

```text
Cloudflare handoff
pricing/seat/referral math
checkout and hosted portal sessions
provider webhook lifecycle
app-owned ledgers
signed entitlement snapshots
invoice/tax/refund/dispute/cancellation/grace
security/privacy/observability/test-live boundaries
provider portability
regional rollout
parent dashboard
support/admin operations
rollout proof and route gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
WP00 Cloudflare handoff proof
runtime checkout/portal proof
webhook lifecycle proof
app-owned ledger proof
signed entitlement snapshot proof
regional provider setup proof
referral qualification approval/proof
parent dashboard targeted proof
support/admin audit/redaction proof
invoice/tax/refund/dispute legal/tax closure
rollout proof aggregation
```

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `payment-subscription-plan`.
- Ownership path: this plan is coordinated via `payment-subscription-plan/AGENTS.md`, `payment-subscription-plan/PLAN_STATE.md`, `payment-subscription-plan/NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and the selected workpack files.

### State

- Current state: the route names the Cloudflare control-plane dependency, the workpack tree, and the proof expectations explicitly, but runtime correctness is still open.
- Current action: keep this file and `payment-subscription-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the proof inventory referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, provider/region/setup blockers remain, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in the proof inventory, assertion IDs, validation logs, rollback/teardown notes, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and the assigned plan workpack.

## PR-ready rule

The whole plan is PR-ready only when WP07 consumes or blocks every earlier workpack proof root and updates PLAN_STATE.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, rollback/teardown notes, no-claim language, and remaining open workpacks listed.
