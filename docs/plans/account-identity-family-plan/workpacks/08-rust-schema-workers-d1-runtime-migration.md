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

## Current status

`validation / bounded source accepted / tests and runtime adapter deferred`

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
  real Workers-D1/DO/KV binding, persistence surface, migration execution
  surface, and storage proof; it consumes this packet's canonical contract.

cloudflare-control-plane-plan WP08:
  Cloudflare test-runner and test-pyramid proof after the WP06 storage packet.
```

## 2026-08-17 independently reviewed source packet

The Rust-owned `v0.7` handoff now requires a canonical household/child/device
binding with pairing, installation, selected route, lifecycle, revocation, and
bounded authority generation. Child-device, pairing, installation, route, and
provider-subject identifiers use guarded deserialization, and Rust/TypeScript
agree on the positive JavaScript-safe generation range. Handoff validation
requires an active provider mapping whose account identity exactly matches the
binding account identity.

`crates/family-identity-core/src/account_identity_authority.rs` owns the
crate-private repository/read port and the non-public trusted result. It rejects
selector mismatch, unpaired or uninstalled devices, inactive lifecycle,
revocation, and invalid generation. Downstream crates cannot implement that
port or fabricate the trusted binding. Independent P0/P1 review accepted this
bounded source packet. No repository adapter or production caller exists yet;
Cloudflare WP06 owns that next source packet. Tests, builds, retained proof,
runtime authority, and DONE remain deferred.

## 2026-08-17 source review result

Status: **bounded source accepted; tests, runtime adapter, and proof deferred.**

The live source audit found that the former optional authority snapshot was a
transport shape, not a trusted binding. The accepted source outcome is:

```text
Rust-owned canonical household/child/device binding
pairing + install + selected route + lifecycle + revocation + authority generation
family-owned fail-closed current-binding read boundary
no caller-supplied DTO, selector, or generated TypeScript value becomes authority
```

Cloudflare WP06 remains the durable repository and production-caller owner.
This packet may define the contract and family-owned read port, but it must not
add D1 storage, a Worker route, provider verification, or a fake local authority
adapter. Existing tests/proof are not evidence for this new packet and every
acceptance row below remains open.

PR #607's TypeScript Cloudflare persistence/D1-test-double work is historical
branch evidence only. It is not an implementation starting point or proof of
this workpack.

## Required proof root

```text
docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/
```

Required artifacts:

```text
00-rust-schema-authority-proof.md
01-account-authority-parity-proof.md
02-account-authority-negative-proof.md
03-redacted-authority-proof.md
04-cloudflare-wp06-wp08-handoff.md
05-no-claim-boundary.md
16-validation-commands.md
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
npm run lint:architecture -- --files crates/schema crates/family-identity-core packages/schema-domain/src/generated-family-references.ts

```

Cloudflare WP06, not this packet, records its exact migration command as
`cd infra/cloudflare && npm exec -c "wrangler d1 migrations apply <account-identity-d1-database> --local"` after it defines the binding and migration.
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

- [x] Workpack id, branch, and exact source owners recorded.
- [x] Required proof artifacts and `16-validation-commands.md` retained.
- [x] Focused command results or precise blockers recorded.
- [x] Cross-plan Cloudflare WP06/WP08 handoffs and no-claim boundary recorded.
- [x] Checklist/PLAN_STATE changes made only for proven rows.

## Accepted replacement source delta

Independent review accepts the replacement Account source at `35edb2830`,
integrated through `e69acf279`. In addition to the prior canonical binding, the
current packet adds strict schema validation modules, non-forgeable capability
issuance, and durable repository/CAS/invariant/read ownership. Generated
Account TypeScript remains derived from the Rust template. Cloudflare WP06 now
contains the source adapter and ordered `0001`-`0004` migration files, but
provider composition, migration execution, expected tests, retained proof, and
deployment remain downstream/open. This is reviewed implementation evidence,
not a new DONE claim.

## Prior narrow completion record

The tracked durable manifest in `docs/proof/account-identity-family-plan/08-rust-schema-workers-d1-runtime-migration/`
records the Rust source/test surface: `crates/schema` owns the generated edge
contract and `crates/family-identity-core` owns account/family authority.

That prior record is intentionally narrow and predates the 2026-08-16 handoff
above. Cloudflare WP06 still owns
D1/DO/KV binding and migration/storage proof; Cloudflare WP08 then owns
runner/integration proof; Account WP06 remains open for final aggregation.
The new handoff is independently accepted as bounded implementation evidence
only. Tests, adapter reachability, validation, proof, and completion remain
open and do not change those ownership or no-claim boundaries.
