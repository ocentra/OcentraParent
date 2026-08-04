<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP08 Rust Schema And Workers-D1 Runtime Migration`
> Kind: assigned implementation and proof workpack.
> Read when: selected from WORKPACK_INDEX.md after the provider/custody decision.
> Stop rule: do not treat a TypeScript D1 test double as real runtime proof or move account authority into Cloudflare scaffolding.
> Proves: only the Rust-owned schema plus real Workers-D1 persistence/migration slice after focused validation and retained proof.
> Does not prove: production authentication, device trust, payment, policy, LAN/remote transport, deployment readiness, or whole-plan completion.
> Proof rule: all WP08 checklist rows remain open until the named proof artifacts and focused commands exist.

<!-- /agent-capsule -->

# WP08 Rust Schema And Workers-D1 Runtime Migration

## Goal

Establish the next executable account-identity slice after the accepted provider
and custody decision: Rust-owned canonical account/family authority schemas,
then a real Cloudflare Workers-D1 persistence/migration implementation that
consumes those schemas without making a TypeScript adapter or D1 test double
the authority.

## Required inputs

```text
AGENTS.md
PLAN_STATE.md
workpacks/00-owner-boundary-proof-gate.md
workpacks/01-auth-provider-decision.md
WORKPACK_INDEX.md
CHECKLIST_INDEX.md (WP08 only)
TEST_PROOF_EXPECTATIONS.md (WP08 only)
PROOF_INDEX.md (WP08 only)
docs/expectations/cloud.md
```

Read the Cloudflare plan only for the explicit worker/runtime handoff. Do not
move account/family ownership into that plan's shared worker scaffold.

## Ownership and handoff

```text
crates/schema or the owning Rust crate:
  canonical account/family/session/device-authority contracts, migration-facing
  shape compatibility, and cross-boundary literals/DTOs.

crates/family-identity-core:
  Rust account/family runtime semantics and parity against the canonical schema.

infra/cloudflare:
  real Workers-D1 binding, persistence adapter, migration execution surface,
  and focused worker integration proof; it consumes canonical Rust-owned
  contracts and does not redefine family authority.

Durable Objects / KV:
  approved short-lived coordination and cache/rate-limit roles only; neither is
  the relational account-family authority.
```

PR #607's TypeScript Cloudflare adapter/D1-test-double work is historical
branch evidence only. It is not an implementation starting point or proof of
this workpack.

## Required proof root

```text
output/account-identity-family-plan-proof/08-rust-schema-workers-d1-runtime-migration/
```

Required artifacts:

```text
00-rust-schema-authority-proof.md
01-migration-safety-proof.md
02-workers-d1-binding-proof.md
03-account-persistence-integration-proof.md
04-runtime-negative-proof.md
05-custody-role-proof.md
06-handoff-and-no-claim-boundary.md
16-validation-commands.log
```

## Acceptance obligations

- [ ] Canonical account/family authority schemas are Rust-owned and consumed without TypeScript ownership drift.
- [ ] A real Workers-D1 binding and persistence adapter exists; local test doubles are insufficient.
- [ ] Migrations cover apply, compatibility, and rollback or an explicit forward-only/custody constraint.
- [ ] Account, household, membership, role, device, invite/recovery, and session authority preserve typed scope and ownership.
- [ ] Wrong-household, revoked, stale, malformed, duplicate, and unavailable-storage cases reject or degrade safely.
- [ ] Durable Objects and KV remain non-authoritative for account/family relational truth.
- [ ] Rust and focused worker validation are retained with compact command logs.
- [ ] The cross-plan Cloudflare handoff and no-claim boundary are retained.
- [ ] Checklist rows are reconciled only after all prior obligations have evidence.

## Focused validation

Use the selected subset from `TEST_PROOF_EXPECTATIONS.md`; at minimum, the
packet must cover the changed canonical Rust owner, family-identity runtime,
and actual Cloudflare worker scope:

```bash
cargo test -p ocentra-family-identity-core account
npm --prefix infra/cloudflare run test:unit
npm --prefix infra/cloudflare run test:integration
npm --prefix infra/cloudflare run test:contract
npm --prefix infra/cloudflare run lint
npm run lint:architecture -- --files crates/schema crates/family-identity-core infra/cloudflare
```

Run a protocol/service consumer only if this workpack changes its typed
handoff. Record a missing runtime, migration environment, or real D1 binding
as a blocker in `16-validation-commands.log`; do not substitute a test double.

## Negative and no-claim boundary

This workpack must distinguish a real D1 runtime from mocks/test doubles and
must reject cross-household, stale/revoked, schema-incompatible, and
unavailable-storage paths. It does not prove provider login, trusted-device
bootstrap, policy/payment authorization, rollout/deployment, or whole-plan
readiness.

## Fill before DONE or PR_READY

- [ ] Workpack id, branch, and exact source owners recorded.
- [ ] Required proof artifacts and `16-validation-commands.log` retained.
- [ ] Focused command results or precise blockers recorded.
- [ ] Cross-plan Cloudflare handoff, custody limits, and no-claim boundary recorded.
- [ ] Checklist/PLAN_STATE changes made only for proven rows.
