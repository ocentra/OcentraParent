<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Next Actions`
> Kind: resume queue and highest-open work.
> Read when: after PLAN_STATE.md.
> Stop rule: pick one workpack; do not broaden into sibling plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: update this file only when queue state changes.

<!-- /agent-capsule -->

# Account Identity Family Plan Next Actions

## How to use

1. Confirm branch and assignment.
2. Open `WORKPACK_INDEX.md`.
3. Select exactly one workpack.
4. Read `workpacks/00-owner-boundary-proof-gate.md` to apply current owner/import/proof rules.
5. Open only the selected workpack.
6. Use `TEST_PROOF_EXPECTATIONS.md` and `PROOF_INDEX.md` for proof obligations.
7. Implement narrow source/docs/test changes.
8. Update `CHECKLIST_INDEX.md`, selected workpack, and `PLAN_STATE.md` only after proof exists.

## Highest-priority queue

Audit snapshot June 17, 2026: WP01 has a docs-only provider/custody proof pack on disk; WP02, WP03, WP04, WP05, and WP07 have prior complete proof roots on disk. WP06 is reopened for a final aggregation rerun after Account WP08 plus Cloudflare WP06/WP08; PR-ready remains false because browser request-safety is still an explicit blocker artifact and the remaining runtime/schema/adjacent execution gaps stay manual-required.

Current routing note: WP08 is the selectable owning packet for the remaining Rust-schema/account-authority gap. Cloudflare WP06 then Cloudflare WP08 own the downstream D1/DO/KV binding, migration, runner, and worker proof. Account WP06 is reopened only to aggregate those exact handoffs; do not reopen WP02-WP05/WP07 unless a proof root drifts or a prior blocker becomes locally owned. Do not route WP08 into the provider-decision-only WP01.

PR #607 is closed without merge. Do not rebase its TypeScript Cloudflare
adapter/D1-test-double slice into this plan. Start with Rust-owned account
schema authority. Then hand the contract to Cloudflare WP06 for D1/DO/KV
persistence/migration and Cloudflare WP08 for runner/integration proof.

### 0. WP08 Rust Schema And Account Authority

Expected result:

```text
Rust-owned canonical account/family authority schema
Rust account-authority parity across household, role, device, invite/recovery, and session semantics
cross-household, stale/revoked, malformed, duplicate, and schema-incompatible negatives
redacted correlated authority proof and retained focused Rust command log
explicit handoff to Cloudflare WP06 then WP08; no worker-runtime claim
```

### 1. WP01 Auth Provider Decision

Expected result:

```text
accepted provider/custody decision with exact D1/DO/KV/R2 ownership split
explicit IdP/Auth.js adapter boundary
degraded/manual-required behavior for provider outage
replacement/migration path
proof root and command log
```

### 2. WP02 Identity Household Role Model

Expected result:

```text
typed account/household/membership/role/device references
role/action/resource matrix with cross-family denial proof
observer/support/admin boundaries
proof root and command log
```

### 3. WP03 Session Token Lifecycle

Expected result:

```text
credential type matrix
browser session lifecycle, replay-safe rotation, revoke/logout, expiry/skew handling
sensitive-action freshness gate
proof root and command log
```

### 4. WP04 Invites Recovery Lifecycle

Expected result:

```text
invite state machine
co-parent/observer/child-device invite scopes
single-use/expiry/revocation/replay proof
recovery flows for forgotten login, lost parent device, compromised-account, child reinstall, transfer
data custody handoff for delete/export
```

### 5. WP05 Device Ownership AuthZ

Expected result:

```text
actor/household/role/device/session/capability authZ matrix
wrong-household and stale/revoked device denial proof
remote view/control capability separation
export/delete and billing owner gates
```

### 6. WP07 Parent Account Family Setup UI

Expected result:

```text
first-run setup state machine
create/join household, child profile, device pair, co-parent/observer invites
honest source/custody/degraded/manual-required UI labels
portal tests or explicit missing UI blocker
```

### 7. WP06 Security Proof And Route Gate

Expected result:

```text
rollout proof pack consumes WP01-WP05/WP07 proof
reopened final gate consumes Account WP08 authority proof plus Cloudflare WP06 storage and Cloudflare WP08 runner/proof handoffs
route sync confirms adjacent plans consume account identity without owning it
remaining manual-required gaps listed
explicit request-safety blocker carried forward without fake-green closure
```

## Blocked execution rules

- Runtime implementation is blocked until WP01 provider/custody decision is complete or the selected workpack explicitly implements that decision.
- UI implementation is blocked until the required contract shape exists or a stub/blocker proof is written.
- Payment/policy/remote/device-trust integration remains blocked until the reopened WP06 route gate consumes green Account WP08 plus Cloudflare WP06/WP08 proof. A precise blocker is recorded for audit only and does not release dependent scheduling.
- Any claim involving secure auth/session requires WP03 and WP06 proof.
- Any selected workpack that conflicts with `workpacks/00-owner-boundary-proof-gate.md` must be updated or blocked before source changes.

## PR readiness guard

A partial PR may be acceptable only when one workpack is fully closed and the report lists remaining open workpacks.

Do not create PR-ready claims from:

```text
docs-only provider discussion without accepted decision
happy-path login flow only
role matrix without cross-family negatives
UI mock without typed contract/source-state proof
invite/recovery flow without replay/rate-limit/enumeration proof
```
