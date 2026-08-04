<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP08 Rust Schema And Account Authority`
> Kind: assigned implementation and proof workpack.
> Read when: selected from WORKPACK_INDEX.md after the provider/custody decision.
> Stop rule: do not implement Cloudflare binding, adapter, migration, or test-runner work in this account-owned packet.
> Proves: only the Rust-owned schema and account-authority parity slice after focused validation and retained proof.
> Does not prove: production authentication, device trust, payment, policy, LAN/remote transport, deployment readiness, or whole-plan completion.
> Proof rule: all WP08 checklist rows remain open until the named proof artifacts and focused commands exist.

<!-- /agent-capsule -->

# WP08 Rust Schema And Account Authority

## Goal

Establish the next executable account-identity slice after the accepted provider
and custody decision: Rust-owned canonical account/family authority schemas and
account-authority parity. Cloudflare persistence consumes this contract in its
own plan; this packet neither implements nor validates the worker runtime.

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

Read the Cloudflare plan only to record the explicit handoff. Do not move
account/family ownership into that plan's shared worker scaffold.

## Ownership and handoff

```text
crates/schema or the owning Rust crate:
  canonical account/family/session/device-authority contracts, migration-facing
  shape compatibility, and cross-boundary literals/DTOs.
  The current TS-edge handoff pattern is `crates/schema/src/family_references_ts.rs`
  and `family_references.template.txt` ->
  `packages/schema-domain/src/generated-family-references.ts`, checked by
  `crates/schema/tests/contract/family_references.rs`. The generated TypeScript
  file is an encoded Rust-contract projection, never an authority owner.

crates/family-identity-core:
  Rust account/family runtime semantics and parity against the canonical schema.

cloudflare-control-plane-plan WP06:
  real Workers-D1/DO/KV binding, persistence adapter, migration execution
  surface, and storage proof; it consumes this packet's canonical contract.

cloudflare-control-plane-plan WP08:
  Cloudflare test-runner and test-pyramid proof after the WP06 storage packet.
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
01-account-authority-parity-proof.md
02-account-authority-negative-proof.md
03-redacted-authority-proof.md
04-cloudflare-wp06-wp08-handoff.md
05-no-claim-boundary.md
16-validation-commands.log
```

## Acceptance obligations

- [ ] Canonical account/family authority schemas are Rust-owned and consumed without TypeScript ownership drift; the exact encoded edge artifact is `packages/schema-domain/src/generated-family-references.ts` from `crates/schema/src/family_references_ts.rs`.
- [ ] Account, household, membership, role, device, invite/recovery, and session authority preserve typed canonical scope and ownership.
- [ ] Wrong-household, revoked, stale, malformed, duplicate, and schema-incompatible authority cases reject or degrade safely.
- [ ] Redacted correlated account-authority proof covers account, household, device, invite, recovery, and session decisions without carrying a worker-runtime claim.
- [ ] The Cloudflare WP06 storage handoff is recorded as a consumer of this contract, not as an Account WP08 implementation duty.
- [ ] The Cloudflare WP08 runner/proof handoff is recorded after Cloudflare WP06, not claimed as Account WP08 validation.
- [ ] Focused Rust validation is retained with compact command logs.
- [ ] The cross-plan no-claim boundary is retained.
- [ ] Checklist rows are reconciled only after all prior obligations have evidence.

## Focused validation

Use the selected subset from `TEST_PROOF_EXPECTATIONS.md`; at minimum, the
packet must cover the changed canonical Rust owner and family-identity runtime:

```bash
# Canonical schema crate plus focused account/family authority coverage.
cargo test -p ocentra-schema --test contract
cargo test -p ocentra-schema --test contract family_references_generated_typescript_matches_checked_in_file
cargo test -p ocentra-family-identity-core household_authority
cargo test -p ocentra-family-identity-core --test unit setup_lifecycle
cargo test -p ocentra-family-identity-core --test unit session_lifecycle
npm run lint:architecture -- --files crates/schema crates/family-identity-core

```

Cloudflare WP06, not this packet, records its exact migration command as
`npm --prefix infra/cloudflare exec -- wrangler d1 migrations apply <account-identity-d1-database> --local` after it defines the binding and migration.
Cloudflare WP08, not this packet, runs Cloudflare module test scripts including
`npm --prefix infra/cloudflare run test:integration`. Record unavailable
Cloudflare handoff proof as a downstream blocker; do not substitute a test
double. Run a protocol/service consumer only if this packet changes its typed
handoff.

The targeted `family_references_generated_typescript_matches_checked_in_file`
contract test is the required Rust-to-TS-edge drift check. A schema-domain build
or edge decode may be added only as a consumer check; neither may define account
authority.

The `setup_lifecycle` and `session_lifecycle` unit targets are minimum WP08
negative-path coverage: invite replay/revocation/wrong-household and recovery
rejection, plus stale/revoked/replayed/wrong-kind session rejection. Do not
substitute the broad household-authority filter for those focused lifecycle
paths.

## Negative and no-claim boundary

This workpack must reject cross-household, stale/revoked, malformed, duplicate,
and schema-incompatible authority paths. Its retained authority proof must use
correlation IDs and redact session credentials, provider claims, invite/recovery
secrets, and child activity data. It does not prove a D1 runtime, provider
login, trusted-device bootstrap, policy/payment authorization, rollout,
deployment, or whole-plan readiness.

## Fill before DONE or PR_READY

- [ ] Workpack id, branch, and exact source owners recorded.
- [ ] Required proof artifacts and `16-validation-commands.log` retained.
- [ ] Focused command results or precise blockers recorded.
- [ ] Cross-plan Cloudflare WP06/WP08 handoffs and no-claim boundary recorded.
- [ ] Checklist/PLAN_STATE changes made only for proven rows.
