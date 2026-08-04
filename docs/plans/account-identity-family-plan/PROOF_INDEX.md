<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `Account Identity Family Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Account Identity Family Plan Proof Index

## Proof roots

```text
output/account-identity-family-plan-proof/01-auth-provider-decision/
output/account-identity-family-plan-proof/08-rust-schema-workers-d1-runtime-migration/
output/account-identity-family-plan-proof/02-identity-household-role-model/
output/account-identity-family-plan-proof/03-session-token-lifecycle/
output/account-identity-family-plan-proof/04-invites-recovery-lifecycle/
output/account-identity-family-plan-proof/05-device-ownership-authz/
output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/
output/account-identity-family-plan-proof/06-security-proof-and-route-gate/
```

## Test result roots

```text
test-results/account-identity-family-plan-provider-decision/
test-results/account-identity-family-plan-rust-schema-workers-d1-runtime-migration/
test-results/account-identity-family-plan-household-role-model/
test-results/account-identity-family-plan-session-token-lifecycle/
test-results/account-identity-family-plan-invites-recovery/
test-results/account-identity-family-plan-device-authz/
test-results/account-identity-family-plan-family-setup-ui/
test-results/account-identity-family-plan-security-route-gate/
```

## Required proof files per workpack

### WP01 Auth Provider Decision

```text
00-provider-decision-record.md
01-provider-rejected-options.md
02-provider-custody-boundary-proof.md
03-custom-claims-data-minimization-proof.md
04-provider-outage-degraded-proof.md
05-migration-path-proof.md
16-validation-commands.log
```

### WP02 Identity Household Role Model

```text
00-identity-entity-model-proof.md
01-role-action-resource-matrix.md
02-membership-state-machine-proof.md
03-cross-family-negative-proof.md
04-observer-read-only-proof.md
05-support-admin-boundary-proof.md
06-audit-event-proof.md
16-validation-commands.log
```

### WP08 Rust Schema And Account Authority

```text
00-rust-schema-authority-proof.md (owner: crates/schema; Rust parity owner: crates/family-identity-core; records crates/schema/src/family_references_ts.rs -> packages/schema-domain/src/generated-family-references.ts plus the checked-in drift assertion)
01-account-authority-parity-proof.md
02-account-authority-negative-proof.md
03-redacted-authority-proof.md
04-cloudflare-wp06-wp08-handoff.md
05-no-claim-boundary.md
16-validation-commands.log
```

### WP03 Session Token Lifecycle

```text
00-credential-type-matrix.md
01-session-lifecycle-proof.md
02-token-expiry-replay-proof.md
03-refresh-revocation-proof.md
04-session-freshness-proof.md
05-csrf-origin-proof.md
06-token-redaction-proof.md
16-validation-commands.log
```

### WP04 Invites Recovery Lifecycle

```text
00-invite-state-machine-proof.md
01-invite-negative-proof.md
02-recovery-state-machine-proof.md
03-recovery-abuse-proof.md
04-delete-export-handoff-proof.md
05-support-recovery-audit-proof.md
16-validation-commands.log
```

### WP05 Device Ownership AuthZ

```text
00-device-authority-matrix.md
01-revoked-device-negative-proof.md
02-wrong-household-negative-proof.md
03-controller-lease-proof.md
04-remote-capability-proof.md
05-export-delete-owner-proof.md
06-billing-owner-proof.md
16-validation-commands.log
```

### WP07 Parent Account Family Setup UI

```text
00-first-run-ui-state-machine.md
01-household-setup-ui-proof.md
02-device-role-ui-proof.md
03-observer-read-only-proof.md
04-recovery-ui-proof.md
05-mobile-parent-child-claim-split-proof.md
06-source-custody-label-proof.md
16-validation-commands.log
```

### WP06 Security Proof And Route Gate

```text
00-security-proof-pack.md
01-authn-negative-proof.md
02-authz-matrix-proof.md
03-token-replay-proof.md
04-recovery-abuse-proof.md
05-origin-csrf-open-redirect-proof.md
06-route-sync-proof.md
07-logging-redaction-proof.md
08-manual-required-gap-register.md
09-account-authority-cloudflare-storage-gate.md
16-validation-commands.log
```

## Command log format

Every proof root must include:

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: account-identity-family-plan
workpack: <WP id and name>
owner: crates/schema | crates/family-identity-core | schema-domain-edge-consumer | family-domain | setup-domain | provisioning-core | portal-domain | apps/portal | protocol/service | docs-only
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <runtime/proof correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <raw stdout/stderr artifact pointer, test-results path, proof file, or n/a>
diagnostics_summary: <short unique failure or proof summary>
redaction_note: <tokens/provider claims/session secrets/child activity data redacted or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw stdout/stderr, Playwright artifacts, screenshots, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show redaction and custody boundaries. Local harness proof may include richer diagnostics, but it still stores raw logs by pointer and keeps plan docs compact.

```text
runtime-safe: no raw session secrets, provider claims, recovery tokens, invite secrets, or child activity data.
local harness: enough file/line/command/artifact context for Codex/MCP/humans to debug without reading terminal walls.
```

## No-claim language

Do not claim:

```text
production auth ready
family setup ready
payment auth ready
policy auth ready
remote access auth ready
device trust ready
secure session complete
provider decision accepted
PR_READY
```

unless the selected workpack proof root proves the claim and WP06 aggregates it when broad readiness is claimed.

Use narrow wording:

```text
provider decision record drafted
custom claims minimization proof passed
cross-family denial unit test passed
session replay negative passed
invite expiry negative passed
device ownership matrix proof passed
first-run setup UI smoke passed
route-sync proof blocked by policy/eventing lane
```
